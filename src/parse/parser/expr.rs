use crate::{
    lex::Token,
    parse::{self, ast, error::Expected, Parser},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    Compare = 1,
    AddSub,
    MulDiv,
    Index,
    Group,
}

fn infix_precedence(token: &Token) -> Option<Precedence> {
    match token {
        Token::Lt | Token::Le | Token::Gt | Token::Ge | Token::Eq | Token::Ne => {
            Some(Precedence::Compare)
        }
        Token::Plus => Some(Precedence::AddSub),
        Token::Hyphen => Some(Precedence::AddSub),
        Token::Asterisk => Some(Precedence::MulDiv),
        Token::Slash => Some(Precedence::MulDiv),
        Token::LeftParen => Some(Precedence::Group),
        Token::LeftBracket => Some(Precedence::Index),
        _ => None,
    }
}

impl Parser {
    pub(super) fn parse_expr(&mut self) -> parse::Result<ast::Expr> {
        self.parse_expr_(None)
    }

    fn parse_expr_again(&mut self, precedence: Precedence) -> parse::Result<ast::Expr> {
        self.parse_expr_(Some(precedence))
    }

    fn parse_expr_(&mut self, precedence: Option<Precedence>) -> parse::Result<ast::Expr> {
        let token = self
            .lexer
            .next()
            .ok_or_else(|| parse::Error::miss(Expected::Expr))??;
        let mut left = self.parse_prefix_expr(token)?;

        loop {
            match self.lexer.peek() {
                None => break,
                Some(Ok(right_token)) => {
                    if precedence < infix_precedence(right_token) {
                        left = self.parse_infix_expr(left)?;
                    } else {
                        break;
                    }
                }
                Some(Err(_)) => return Err(self.lexer.next().unwrap().unwrap_err().into()),
            }
        }

        Ok(left)
    }

    fn parse_prefix_expr(&mut self, token: Token) -> parse::Result<ast::Expr> {
        match token {
            Token::True => Ok(ast::Expr::Bool(true)),
            Token::False => Ok(ast::Expr::Bool(false)),
            Token::Int(value) => Ok(ast::Expr::Int(value)),
            Token::Str(value) => Ok(ast::Expr::Str(value.into())),
            Token::LeftParen => {
                let expr = self.parse_expr()?;
                self.expect_token(Token::RightParen)?;
                Ok(expr)
            }
            Token::Fn => self
                .parse_function_expression()
                .map(std::rc::Rc::new)
                .map(ast::Expr::Closure),
            Token::Ident(ident) => Ok(ast::Expr::Ident(ast::Ident::new(ident.into()))),
            Token::If => self.parse_if_expr().map(Box::new).map(ast::Expr::If),
            Token::LeftBracket => self
                .parse_separated_with(Token::Comma, Token::RightBracket, |parser| {
                    parser.parse_expr()
                })
                .map(Vec::into_boxed_slice)
                .map(ast::Expr::Seq),
            Token::Hyphen => self
                .parse_expr()
                .map(|expr| ast::Expr::Unary(ast::UnaryOp::Neg, Box::new(expr))),
            Token::Bang => self
                .parse_expr()
                .map(|expr| ast::Expr::Unary(ast::UnaryOp::Not, Box::new(expr))),
            Token::LeftBrace => {
                // map literal
                //
                // { <key-expr>: <value-expr>, ... }

                self.parse_separated_with(Token::Comma, Token::RightBrace, |parse| {
                    let key = parse.parse_expr()?;
                    parse.expect_token(Token::Colon)?;
                    let value = parse.parse_expr()?;

                    Ok((key, value))
                })
                .map(Vec::into_boxed_slice)
                .map(ast::Expr::Map)
            }
            tk => Err(parse::Error::mismatch(tk, Expected::Expr)),
        }
    }

    fn parse_infix_expr(&mut self, left: ast::Expr) -> parse::Result<ast::Expr> {
        fn binary_expr(
            self_: &mut Parser,
            left: ast::Expr,
            op: ast::BinaryOp,
            precedence: Precedence,
        ) -> parse::Result<ast::Expr> {
            let right = self_.parse_expr_again(precedence)?;

            Ok(ast::Expr::Binary(Box::new(left), op, Box::new(right)))
        }

        match self
            .lexer
            .next()
            .ok_or_else(|| parse::Error::miss(Expected::Expr))??
        {
            Token::Plus => binary_expr(self, left, ast::BinaryOp::Add, Precedence::AddSub),
            Token::Hyphen => binary_expr(self, left, ast::BinaryOp::Sub, Precedence::AddSub),
            Token::Asterisk => binary_expr(self, left, ast::BinaryOp::Mul, Precedence::MulDiv),
            Token::Slash => binary_expr(self, left, ast::BinaryOp::Div, Precedence::MulDiv),
            Token::Eq => binary_expr(self, left, ast::BinaryOp::Eq, Precedence::Compare),
            Token::Ne => binary_expr(self, left, ast::BinaryOp::Ne, Precedence::Compare),
            Token::Lt => binary_expr(self, left, ast::BinaryOp::Lt, Precedence::Compare),
            Token::Le => binary_expr(self, left, ast::BinaryOp::Le, Precedence::Compare),
            Token::Gt => binary_expr(self, left, ast::BinaryOp::Gt, Precedence::Compare),
            Token::Ge => binary_expr(self, left, ast::BinaryOp::Ge, Precedence::Compare),
            Token::LeftParen => {
                // call expr
                //
                // <f>(<arg>...)
                self.parse_call_expr(left)
            }
            Token::LeftBracket => {
                // index expr
                //
                // <target>[<key>]

                let key = self.parse_expr()?;
                self.expect_token(Token::RightBracket)?;
                Ok(ast::Expr::Index(Box::new(left), Box::new(key)))
            }
            tk => Err(parse::Error::mismatch(tk, Expected::Expr)),
        }
    }
}

impl Parser {
    // with token `fn` skipped
    fn parse_function_expression(&mut self) -> parse::Result<ast::Closure> {
        // fn(<parameter...>) { <statement...> }

        self.expect_token(Token::LeftParen)?;

        let parameters = self.parse_separated_with(Token::Comma, Token::RightParen, |parser| {
            parser.expect_ident()
        })?;

        let body = self.parse_block()?;

        Ok(ast::Closure {
            parameters: parameters.into_boxed_slice(),
            body,
        })
    }

    // with token `(` skipped
    fn parse_call_expr(&mut self, f: ast::Expr) -> parse::Result<ast::Expr> {
        let args = self
            .parse_separated_with(Token::Comma, Token::RightParen, |parser| {
                parser.parse_expr()
            })
            .map(Vec::into_boxed_slice)?;

        Ok(ast::Expr::Call(Box::new(f), args))
    }

    // with token `if` skipped
    fn parse_if_expr(&mut self) -> parse::Result<ast::If> {
        let condition = self.parse_expr()?;

        let consequence = self.parse_block()?;

        let alternative = if let Some(&Ok(Token::Else)) = self.lexer.peek() {
            self.expect_token(Token::Else)?;

            Some(self.parse_block()?)
        } else {
            None
        };

        Ok(ast::If {
            condition,
            consequence,
            alternative,
        })
    }
}

impl Parser {
    fn parse_separated_with<F, T>(
        &mut self,
        separater: Token,
        end: Token,
        parse_element: F,
    ) -> parse::Result<Vec<T>>
    where
        F: Fn(&mut Parser) -> parse::Result<T>,
    {
        if let Some(Ok(tk)) = self.lexer.peek() {
            if tk == &end {
                // empty

                self.lexer.next().unwrap().unwrap();

                return Ok(Vec::new());
            }
        }

        let mut elements = Vec::new();

        loop {
            elements.push(parse_element(self)?);

            match self.lexer.next() {
                Some(Ok(tk)) => {
                    if tk == separater {
                        // continue
                    } else if tk == end {
                        return Ok(elements);
                    } else {
                        return Err(parse::Error::mismatch(tk, Expected::Token(end)));
                    }
                }
                Some(Err(err)) => return Err(err.into()),
                None => {
                    return Err(parse::Error::miss(Expected::Token(end)));
                }
            }
        }
    }

    fn parse_block(&mut self) -> parse::Result<ast::Block> {
        self.expect_token(Token::LeftBrace)?;

        let mut nodes = Vec::new();

        loop {
            match self
                .lexer
                .peek()
                .ok_or_else(|| parse::Error::miss(Expected::Token(Token::RightBrace)))?
            {
                Err(_) => return Err(self.lexer.next().unwrap().unwrap_err().into()),
                Ok(Token::RightBrace) => {
                    self.lexer.next().unwrap().unwrap(); // skip `}`

                    return Ok(ast::Block {
                        nodes: nodes.into_boxed_slice(),
                    });
                }
                Ok(_) => {
                    nodes.push(self.next().unwrap()?);
                }
            }
        }
    }
}
