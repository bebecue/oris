use crate::{
    lex::token::{self, Token},
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

fn infix_precedence(kind: &token::Kind) -> Option<Precedence> {
    match kind {
        token::Kind::Lt
        | token::Kind::Le
        | token::Kind::Gt
        | token::Kind::Ge
        | token::Kind::Eq
        | token::Kind::Ne => Some(Precedence::Compare),
        token::Kind::Plus => Some(Precedence::AddSub),
        token::Kind::Hyphen => Some(Precedence::AddSub),
        token::Kind::Asterisk => Some(Precedence::MulDiv),
        token::Kind::Slash => Some(Precedence::MulDiv),
        token::Kind::LeftParen => Some(Precedence::Group),
        token::Kind::LeftBracket => Some(Precedence::Index),
        _ => None,
    }
}

impl<'a> Parser<'a> {
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
            .ok_or_else(|| parse::Error::Incomplete(Box::new(Expected::Expr)))??;

        let mut left = self.parse_prefix_expr(token)?;

        loop {
            match self.lexer.peek() {
                None => break,
                Some(Ok(right_token)) => {
                    if precedence < infix_precedence(&right_token.kind) {
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
        match token.kind {
            token::Kind::True => Ok(ast::Expr::Bool(ast::Bool {
                pos: token.pos,
                value: true,
            })),
            token::Kind::False => Ok(ast::Expr::Bool(ast::Bool {
                pos: token.pos,
                value: false,
            })),
            token::Kind::Int(value) => Ok(ast::Expr::Int(ast::Int {
                pos: token.pos,
                value,
            })),
            token::Kind::Str(value) => Ok(ast::Expr::Str(ast::Str {
                pos: token.pos,
                value: value.into(),
            })),
            token::Kind::LeftParen => {
                let expr = self.parse_expr()?;
                self.expect_token(token::Kind::RightParen)?;
                Ok(expr)
            }
            token::Kind::Fn => self
                .parse_function_expression(token.pos)
                .map(std::rc::Rc::new)
                .map(ast::Expr::Closure),
            token::Kind::Ident(ident) => Ok(ast::Expr::Ident(ast::Ident::from_src(
                token.pos,
                ident.into(),
            ))),
            token::Kind::If => self
                .parse_if_expr(token.pos)
                .map(Box::new)
                .map(ast::Expr::If),
            token::Kind::LeftBracket => self
                .parse_separated_with(token::Kind::Comma, token::Kind::RightBracket, |parser| {
                    parser.parse_expr()
                })
                .map(Vec::into_boxed_slice)
                .map(|elements| {
                    ast::Expr::Seq(ast::Seq {
                        pos: token.pos,
                        elements,
                    })
                }),
            token::Kind::Hyphen => self.parse_expr().map(|expr| {
                ast::Expr::Unary(Box::new(ast::Unary {
                    pos: token.pos,
                    op: ast::UnaryOp::Neg,
                    value: expr,
                }))
            }),
            token::Kind::Bang => self.parse_expr().map(|expr| {
                ast::Expr::Unary(Box::new(ast::Unary {
                    pos: token.pos,
                    op: ast::UnaryOp::Not,
                    value: expr,
                }))
            }),
            token::Kind::LeftBrace => {
                // map literal
                //
                // { <key-expr>: <value-expr>, ... }

                self.parse_separated_with(token::Kind::Comma, token::Kind::RightBrace, |parser| {
                    let key = parser.parse_expr()?;
                    parser.expect_token(token::Kind::Colon)?;
                    let value = parser.parse_expr()?;
                    Ok((key, value))
                })
                .map(Vec::into_boxed_slice)
                .map(|entries| {
                    ast::Expr::Map(ast::Map {
                        pos: token.pos,
                        entries,
                    })
                })
            }
            _ => Err(parse::Error::Mismatch(Box::new(parse::error::Mismatch {
                left: token,
                right: Expected::Expr,
            }))),
        }
    }

    fn parse_infix_expr(&mut self, left: ast::Expr) -> parse::Result<ast::Expr> {
        fn binary_expr(
            self_: &mut Parser<'_>,
            pos: usize,
            left: ast::Expr,
            op: ast::BinaryOp,
            precedence: Precedence,
        ) -> parse::Result<ast::Expr> {
            let right = self_.parse_expr_again(precedence)?;

            Ok(ast::Expr::Binary(Box::new(ast::Binary {
                pos,
                left,
                op,
                right,
            })))
        }

        // the next token should be an infix operator if caller calls this function
        let tk = self.lexer.next().unwrap().unwrap();

        match tk.kind {
            token::Kind::Plus => {
                binary_expr(self, tk.pos, left, ast::BinaryOp::Add, Precedence::AddSub)
            }
            token::Kind::Hyphen => {
                binary_expr(self, tk.pos, left, ast::BinaryOp::Sub, Precedence::AddSub)
            }
            token::Kind::Asterisk => {
                binary_expr(self, tk.pos, left, ast::BinaryOp::Mul, Precedence::MulDiv)
            }
            token::Kind::Slash => {
                binary_expr(self, tk.pos, left, ast::BinaryOp::Div, Precedence::MulDiv)
            }
            token::Kind::Eq => {
                binary_expr(self, tk.pos, left, ast::BinaryOp::Eq, Precedence::Compare)
            }
            token::Kind::Ne => {
                binary_expr(self, tk.pos, left, ast::BinaryOp::Ne, Precedence::Compare)
            }
            token::Kind::Lt => {
                binary_expr(self, tk.pos, left, ast::BinaryOp::Lt, Precedence::Compare)
            }
            token::Kind::Le => {
                binary_expr(self, tk.pos, left, ast::BinaryOp::Le, Precedence::Compare)
            }
            token::Kind::Gt => {
                binary_expr(self, tk.pos, left, ast::BinaryOp::Gt, Precedence::Compare)
            }
            token::Kind::Ge => {
                binary_expr(self, tk.pos, left, ast::BinaryOp::Ge, Precedence::Compare)
            }
            token::Kind::LeftParen => {
                // call expr
                //
                // <f>(<arg>...)
                self.parse_call_expr(tk.pos, left)
            }
            token::Kind::LeftBracket => {
                // index expr
                //
                // <base>[<subscript>]

                let subscript = self.parse_expr()?;
                self.expect_token(token::Kind::RightBracket)?;
                Ok(ast::Expr::Index(Box::new(ast::Index {
                    pos: tk.pos,
                    base: left,
                    subscript,
                })))
            }
            _ => Err(parse::Error::Mismatch(Box::new(parse::error::Mismatch {
                left: tk,
                right: Expected::Expr,
            }))),
        }
    }
}

impl<'a> Parser<'a> {
    // with token `fn` skipped
    fn parse_function_expression(&mut self, pos: usize) -> parse::Result<ast::Closure> {
        // fn(<parameter...>) { <statement...> }

        self.expect_token(token::Kind::LeftParen)?;

        let parameters =
            self.parse_separated_with(token::Kind::Comma, token::Kind::RightParen, |parser| {
                parser.expect_ident()
            })?;

        let body = self.parse_block()?;

        Ok(ast::Closure {
            pos,
            parameters: parameters.into_boxed_slice(),
            body,
        })
    }

    // with token `(` skipped
    fn parse_call_expr(&mut self, pos: usize, f: ast::Expr) -> parse::Result<ast::Expr> {
        let args = self
            .parse_separated_with(token::Kind::Comma, token::Kind::RightParen, |parser| {
                parser.parse_expr()
            })
            .map(Vec::into_boxed_slice)?;

        Ok(ast::Expr::Call(Box::new(ast::Call {
            pos,
            target: f,
            args,
        })))
    }

    // with token `if` skipped
    fn parse_if_expr(&mut self, pos: usize) -> parse::Result<ast::If> {
        let condition = self.parse_expr()?;

        let consequence = self.parse_block()?;

        let alternative = if let Some(_tk) = self
            .lexer
            .next_if(|tk| matches!(tk, Ok(tk) if tk.kind == token::Kind::Else))
        {
            Some(self.parse_block()?)
        } else {
            None
        };

        Ok(ast::If {
            pos,
            condition,
            consequence,
            alternative,
        })
    }
}

impl<'a> Parser<'a> {
    fn parse_separated_with<F, T>(
        &mut self,
        separater: token::Kind,
        end: token::Kind,
        parse_element: F,
    ) -> parse::Result<Vec<T>>
    where
        F: Fn(&mut Parser<'a>) -> parse::Result<T>,
    {
        if let Some(_) = self
            .lexer
            .next_if(|tk| matches!(tk, Ok(tk) if tk.kind == end))
        {
            // empty

            return Ok(Vec::new());
        }

        let mut elements = Vec::new();

        loop {
            elements.push(parse_element(self)?);

            match self.lexer.next() {
                Some(Ok(tk)) => {
                    if tk.kind == separater {
                        // continue
                    } else if tk.kind == end {
                        return Ok(elements);
                    } else {
                        return Err(parse::Error::Mismatch(Box::new(parse::error::Mismatch {
                            left: tk,
                            right: Expected::Token(end),
                        })));
                    }
                }
                Some(Err(err)) => return Err(err.into()),
                None => {
                    return Err(parse::Error::Incomplete(Box::new(Expected::Token(end))));
                }
            }
        }
    }

    fn parse_block(&mut self) -> parse::Result<ast::Block> {
        let pos = self.expect_token(token::Kind::LeftBrace)?;

        let mut nodes = Vec::new();

        loop {
            match self.lexer.peek().ok_or_else(|| {
                parse::Error::Incomplete(Box::new(Expected::Token(token::Kind::RightBrace)))
            })? {
                Err(_) => return Err(self.lexer.next().unwrap().unwrap_err().into()),
                Ok(tk) if tk.kind == token::Kind::RightBrace => {
                    self.lexer.next().unwrap().unwrap(); // skip `}`

                    return Ok(ast::Block {
                        pos,
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
