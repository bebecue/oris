mod expr;

use crate::{
    lex::{self, token},
    parse::{self, ast, error::Expected},
};

pub(crate) struct Parser<'a> {
    lexer: lex::Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(lexer: lex::Lexer<'a>) -> Self {
        Self { lexer }
    }
}

impl Iterator for Parser<'_> {
    type Item = parse::Result<ast::Node>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lexer.peek()? {
            Err(_) => Some(Err(self.lexer.next().unwrap().unwrap_err().into())),
            Ok(token) => match token.kind {
                token::Kind::Let => Some(self.parse_let_stmt().map(ast::Node::Stmt)),
                token::Kind::Return => Some(self.parse_return_stmt().map(ast::Node::Stmt)),
                _ => Some(self.parse_standalone_expr().map(ast::Node::Expr)),
            },
        }
    }
}

impl<'a> Parser<'a> {
    fn parse_let_stmt(&mut self) -> parse::Result<ast::Stmt> {
        let pos = self.expect_token(token::Kind::Let)?;
        let ident = self.expect_ident()?;
        self.expect_token(token::Kind::Assign)?;
        let value = self.parse_expr()?;
        self.skip_optional_semicolon();
        Ok(ast::Stmt::Let(ast::Let { pos, ident, value }))
    }

    fn parse_return_stmt(&mut self) -> parse::Result<ast::Stmt> {
        // `return ;`
        // `return <expr>`
        // `return <expr> ;`

        let pos = self.expect_token(token::Kind::Return)?;

        let value = match self.lexer.peek() {
            Some(Ok(tk)) if tk.kind == token::Kind::Semicolon => {
                let _ = self.lexer.next();
                None
            }
            _ => {
                let expr = self.parse_expr()?;
                self.skip_optional_semicolon();
                Some(expr)
            }
        };

        Ok(ast::Stmt::Return(ast::Return { pos, value }))
    }

    fn parse_standalone_expr(&mut self) -> parse::Result<ast::Expr> {
        // `<expr>`
        // `<expr> ;`

        let expr = self.parse_expr()?;

        self.skip_optional_semicolon();

        Ok(expr)
    }
}

impl<'a> Parser<'a> {
    fn expect_ident(&mut self) -> parse::Result<ast::Ident> {
        let pos = self.lexer.pos();

        match self.lexer.next() {
            Some(Ok(tk)) => match tk.kind {
                token::Kind::Ident => {
                    let sym = self.lexer.lex_atom(tk.pos);
                    Ok(ast::Ident::from_str(tk.pos, sym))
                }
                _ => Err(parse::Error::Mismatch(parse::error::Mismatch {
                    left: tk,
                    right: Expected::Token(token::Kind::Ident),
                })),
            },
            Some(Err(err)) => Err(err.into()),
            None => Err(parse::Error::Incomplete(parse::error::Incomplete {
                pos,
                expected: Expected::Token(token::Kind::Ident),
            })),
        }
    }

    fn expect_token(&mut self, expected: token::Kind) -> parse::Result<usize> {
        let pos = self.lexer.pos();

        match self.lexer.next() {
            Some(Ok(next_token)) => {
                if next_token.kind == expected {
                    Ok(next_token.pos)
                } else {
                    Err(parse::Error::Mismatch(parse::error::Mismatch {
                        left: next_token,
                        right: Expected::Token(expected),
                    }))
                }
            }
            Some(Err(err)) => Err(err.into()),
            None => Err(parse::Error::Incomplete(parse::error::Incomplete {
                pos,
                expected: Expected::Token(expected),
            })),
        }
    }

    fn skip_optional_semicolon(&mut self) {
        if let Some(Ok(tk)) = self.lexer.peek() {
            if tk.kind == token::Kind::Semicolon {
                let _ = self.lexer.next();
            }
        }
    }
}
