use crate::lex::{
    self,
    token::{Kind, Token},
};

pub(crate) struct Lexer<'a> {
    input: &'a [u8],
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a [u8]) -> Self {
        Self { input, cursor: 0 }
    }
}

impl Iterator for Lexer<'_> {
    type Item = lex::Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.skip_writespaces();

            let pos = self.cursor;

            let kind = match self.read_byte()? {
                b',' => Ok(Kind::Comma),
                b':' => Ok(Kind::Colon),
                b';' => Ok(Kind::Semicolon),
                b'(' => Ok(Kind::LeftParen),
                b')' => Ok(Kind::RightParen),
                b'[' => Ok(Kind::LeftBracket),
                b']' => Ok(Kind::RightBracket),
                b'{' => Ok(Kind::LeftBrace),
                b'}' => Ok(Kind::RightBrace),
                b'+' => Ok(Kind::Plus),
                b'-' => Ok(Kind::Hyphen),
                b'*' => Ok(Kind::Asterisk),
                b'/' => Ok(Kind::Slash),
                b'"' => self.read_string().map(Kind::Str),
                b'0'..=b'9' => {
                    self.unwind();
                    self.read_int().map(Kind::Int)
                }
                b'=' => match self.read_byte() {
                    Some(b'=') => Ok(Kind::Eq),
                    Some(_) => {
                        self.unwind();
                        Ok(Kind::Assign)
                    }
                    None => Ok(Kind::Assign),
                },
                b'!' => match self.read_byte() {
                    Some(b'=') => Ok(Kind::Ne),
                    Some(_) => {
                        self.unwind();
                        Ok(Kind::Bang)
                    }
                    None => Ok(Kind::Bang),
                },
                b'<' => match self.read_byte() {
                    Some(b'=') => Ok(Kind::Le),
                    Some(_) => {
                        self.unwind();
                        Ok(Kind::Lt)
                    }
                    None => Ok(Kind::Lt),
                },
                b'>' => match self.read_byte() {
                    Some(b'=') => Ok(Kind::Ge),
                    Some(_) => {
                        self.unwind();
                        Ok(Kind::Gt)
                    }
                    None => Ok(Kind::Gt),
                },
                b'#' => {
                    self.skip_comment();

                    continue;
                }
                b => {
                    self.unwind();

                    if is_atom_head(b) {
                        let atom = self.read_atom();

                        if let Some(k) = to_keyword(atom) {
                            Ok(k)
                        } else {
                            // identifier
                            debug_assert!(atom.is_ascii());

                            let ident = std::str::from_utf8(atom).unwrap();
                            Ok(Kind::Ident(ident.into()))
                        }
                    } else {
                        break Some(Err(lex::Error {
                            pos,
                            kind: lex::error::Kind::Unexpected(b),
                        }));
                    }
                }
            };

            break Some(kind.map(|kind| Token { pos, kind }));
        }
    }
}

impl Lexer<'_> {
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

impl<'a> Lexer<'a> {
    // ident or keyword
    fn read_atom(&mut self) -> &'a [u8] {
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
        let start_pos = self.cursor;

        let mut num: i32 = 0;

        while let Some(&b) = self.input.get(self.cursor) {
            match b {
                b'0'..=b'9' => {
                    num = num
                        .checked_mul(10)
                        .and_then(|num| num.checked_add(i32::from(b - b'0')))
                        .ok_or(lex::Error {
                            pos: start_pos,
                            kind: lex::error::Kind::Overflow,
                        })?;

                    self.cursor += 1;
                }
                _ if is_atom_tail(b) => {
                    return Err(lex::Error {
                        pos: start_pos,
                        kind: lex::error::Kind::BadDigit(b),
                    });
                }
                _ => break,
            }
        }

        Ok(num)
    }

    // with '"' skipped
    fn read_string(&mut self) -> lex::Result<Box<str>> {
        let string_start_pos = self.cursor - 1;

        let len = self.input[self.cursor..]
            .iter()
            .position(|b| *b == b'"')
            .ok_or(lex::Error {
                pos: string_start_pos,
                kind: lex::error::Kind::Quote,
            })?;

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

fn to_keyword(atom: &[u8]) -> Option<Kind> {
    match atom {
        b"let" => Some(Kind::Let),
        b"true" => Some(Kind::True),
        b"false" => Some(Kind::False),
        b"fn" => Some(Kind::Fn),
        b"if" => Some(Kind::If),
        b"else" => Some(Kind::Else),
        b"return" => Some(Kind::Return),
        _ => None,
    }
}
