use crate::lex::{self, Token};

pub(crate) struct Lexer {
    input: Box<[u8]>,
    cursor: usize,
}

impl Lexer {
    pub(crate) fn new(input: Box<[u8]>) -> Self {
        Self { input, cursor: 0 }
    }
}

impl Iterator for Lexer {
    type Item = lex::Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_writespaces();

        match self.read_byte()? {
            b',' => Some(Ok(Token::Comma)),
            b':' => Some(Ok(Token::Colon)),
            b';' => Some(Ok(Token::Semicolon)),
            b'(' => Some(Ok(Token::LeftParen)),
            b')' => Some(Ok(Token::RightParen)),
            b'[' => Some(Ok(Token::LeftBracket)),
            b']' => Some(Ok(Token::RightBracket)),
            b'{' => Some(Ok(Token::LeftBrace)),
            b'}' => Some(Ok(Token::RightBrace)),
            b'+' => Some(Ok(Token::Plus)),
            b'-' => Some(Ok(Token::Hyphen)),
            b'*' => Some(Ok(Token::Asterisk)),
            b'/' => Some(Ok(Token::Slash)),
            b'"' => Some(self.read_string().map(Token::Str)),
            b'0'..=b'9' => {
                self.unwind();
                Some(self.read_int().map(Token::Int))
            }
            b'=' => match self.read_byte() {
                Some(b'=') => Some(Ok(Token::Eq)),
                Some(_) => {
                    self.unwind();
                    Some(Ok(Token::Assign))
                }
                None => Some(Ok(Token::Assign)),
            },
            b'!' => match self.read_byte() {
                Some(b'=') => Some(Ok(Token::Ne)),
                Some(_) => {
                    self.unwind();
                    Some(Ok(Token::Bang))
                }
                None => Some(Ok(Token::Bang)),
            },
            b'<' => match self.read_byte() {
                Some(b'=') => Some(Ok(Token::Le)),
                Some(_) => {
                    self.unwind();
                    Some(Ok(Token::Lt))
                }
                None => Some(Ok(Token::Lt)),
            },
            b'>' => match self.read_byte() {
                Some(b'=') => Some(Ok(Token::Ge)),
                Some(_) => {
                    self.unwind();
                    Some(Ok(Token::Gt))
                }
                None => Some(Ok(Token::Gt)),
            },
            b'#' => {
                self.skip_comment();

                self.next()
            }
            b => {
                self.unwind();

                if is_atom_head(b) {
                    let atom = self.read_atom();

                    if let Some(k) = to_keyword(atom) {
                        Some(Ok(k))
                    } else {
                        // identifier
                        debug_assert!(atom.is_ascii());

                        let ident = std::str::from_utf8(atom).unwrap();
                        Some(Ok(Token::Ident(ident.into())))
                    }
                } else {
                    Some(Err(lex::Error::Unexpected(b)))
                }
            }
        }
    }
}

impl Lexer {
    fn read_byte(&mut self) -> Option<u8> {
        self.input.get(self.cursor).map(|b| {
            self.cursor += 1;
            *b
        })
    }

    fn skip_writespaces(&mut self) {
        while let Some(b) = self.input.get(self.cursor) {
            if b.is_ascii_whitespace() {
                self.cursor += 1;
            } else {
                break;
            }
        }
    }

    fn unwind(&mut self) {
        self.cursor -= 1;
    }
}

impl Lexer {
    // ident or keyword
    fn read_atom(&mut self) -> &[u8] {
        let start_pos = self.cursor;

        let mut end_pos = start_pos + 1;

        while let Some(&b) = self.input.get(end_pos) {
            if is_atom_tail(b) {
                end_pos += 1;
            } else {
                break;
            }
        }

        self.cursor = end_pos;

        self.input.get(start_pos..end_pos).unwrap()
    }

    fn read_int(&mut self) -> lex::Result<i32> {
        let mut num: i32 = 0;

        while let Some(&b) = self.input.get(self.cursor) {
            match b {
                b'0'..=b'9' => {
                    num = num.checked_mul(10).ok_or(lex::Error::Overflow)?;
                    num = num
                        .checked_add(i32::from(b - b'0'))
                        .ok_or(lex::Error::Overflow)?;

                    self.cursor += 1;
                }
                _ if is_atom_tail(b) => return Err(lex::Error::BadDigit(b)),
                _ => break,
            }
        }

        Ok(num)
    }

    // with '"' skipped
    fn read_string(&mut self) -> lex::Result<Box<str>> {
        let len = self.input[self.cursor..]
            .iter()
            .position(|b| *b == b'"')
            .ok_or(lex::Error::Quote)?;

        let s = &self.input[self.cursor..][..len];
        let s = std::str::from_utf8(s).unwrap().into();
        self.cursor += len;
        self.cursor += 1; // skip the left `"`

        Ok(s)
    }

    fn skip_comment(&mut self) {
        match self.input[self.cursor..].iter().position(|b| *b == b'\n') {
            Some(pos) => {
                self.cursor += pos + 1;
            }
            None => self.cursor = self.input.len(),
        }
    }
}

fn is_atom_head(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

fn is_atom_tail(b: u8) -> bool {
    is_atom_head(b) || b.is_ascii_digit()
}

fn to_keyword(atom: &[u8]) -> Option<Token> {
    match atom {
        b"let" => Some(Token::Let),
        b"true" => Some(Token::True),
        b"false" => Some(Token::False),
        b"fn" => Some(Token::Fn),
        b"if" => Some(Token::If),
        b"else" => Some(Token::Else),
        b"return" => Some(Token::Return),
        _ => None,
    }
}
