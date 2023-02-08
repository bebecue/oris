mod expr;

use crate::{
    lex::{self, Token},
    parse::{self, ast, error::Expected},
};

pub(crate) struct Parser {
    lexer: std::iter::Peekable<lex::Lexer>,
}

impl Parser {
    pub(crate) fn new(lexer: lex::Lexer) -> Self {
        Self {
            lexer: lexer.peekable(),
        }
    }
}

impl Iterator for Parser {
    type Item = parse::Result<ast::Node>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lexer.peek()? {
            Err(_) => Some(Err(self.lexer.next().unwrap().unwrap_err().into())),
            Ok(token) => match token {
                Token::Let => Some(self.parse_let_stmt().map(ast::Node::Stmt)),
                Token::Return => Some(self.parse_return_stmt().map(ast::Node::Stmt)),
                _ => Some(self.parse_standalone_expr().map(ast::Node::Expr)),
            },
        }
    }
}

impl Parser {
    fn parse_let_stmt(&mut self) -> parse::Result<ast::Stmt> {
        self.expect_token(Token::Let)?;
        let ident = self.expect_ident()?;
        self.expect_token(Token::Assign)?;
        let value = self.parse_expr()?;
        self.skip_optional_semicolon();
        Ok(ast::Stmt::Let(ident, value))
    }

    fn parse_return_stmt(&mut self) -> parse::Result<ast::Stmt> {
        // `return ;`
        // `return <expr>`
        // `return <expr> ;`

        self.expect_token(Token::Return)?;

        let value = if let Some(&Ok(Token::Semicolon)) = self.lexer.peek() {
            self.lexer.next().unwrap().unwrap(); // skip `;`
            None
        } else {
            let expr = self.parse_expr()?;
            self.skip_optional_semicolon();
            Some(expr)
        };

        Ok(ast::Stmt::Return(value))
    }

    fn parse_standalone_expr(&mut self) -> parse::Result<ast::Expr> {
        // `<expr>`
        // `<expr> ;`

        let expr = self.parse_expr()?;

        self.skip_optional_semicolon();

        Ok(expr)
    }
}

impl Parser {
    fn expect_ident(&mut self) -> parse::Result<ast::Ident> {
        match self.lexer.next() {
            Some(Ok(Token::Ident(ident))) => Ok(ast::Ident::new(ident.into())),
            Some(Ok(not_ident)) => Err(parse::Error::mismatch(not_ident, Expected::Ident)),
            Some(Err(err)) => Err(err.into()),
            None => Err(parse::Error::miss(Expected::Ident)),
        }
    }

    fn expect_token(&mut self, expected: Token) -> parse::Result<()> {
        match self.lexer.next() {
            Some(Ok(next_token)) => {
                if next_token == expected {
                    Ok(())
                } else {
                    Err(parse::Error::mismatch(
                        next_token,
                        Expected::Token(expected),
                    ))
                }
            }
            Some(Err(err)) => Err(err.into()),
            None => Err(parse::Error::miss(Expected::Token(expected))),
        }
    }

    fn skip_optional_semicolon(&mut self) {
        if let Some(&Ok(Token::Semicolon)) = self.lexer.peek() {
            self.lexer.next().unwrap().unwrap();
        }
    }
}
