use crate::lex::{
    self,
    token::{Kind, Token},
};

pub(crate) struct Lexer<'a> {
    input: &'a [u8],
    cursor: usize,
    peeked: Option<lex::Result<Token>>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a [u8]) -> Self {
        Self {
            input,
            cursor: 0,
            peeked: None,
        }
    }

    pub(crate) fn pos(&self) -> usize {
        self.cursor
    }

    pub(crate) fn peek(&mut self) -> Option<&lex::Result<Token>> {
        if self.peeked.is_none() {
            self.peeked = self.next();
        }

        self.peeked.as_ref()
    }

    pub(crate) fn next(&mut self) -> Option<lex::Result<Token>> {
        if let Some(peeked) = self.peeked.take() {
            return Some(peeked);
        }

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
                b'"' => {
                    self.lex_str(pos)
                        .map(str::len) // call map() to drop mutable borrow to self
                        .map(|str_len| {
                            self.cursor += str_len;
                            self.cursor += 1; // skip the left `"`
                            Kind::Str
                        })
                }
                b'0'..=b'9' => {
                    self.unwind();
                    self.lex_int(pos).map(|(_, new_cursor)| {
                        self.cursor = new_cursor;
                        Kind::Int
                    })
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
                        let atom = self.lex_atom(pos);
                        self.cursor += atom.len();

                        if let Some(k) = to_keyword(atom) {
                            Ok(k)
                        } else {
                            // identifier
                            debug_assert!(atom.is_ascii());

                            Ok(Kind::Ident)
                        }
                    } else {
                        break Some(Err(lex::Error {
                            pos,
                            kind: lex::error::Kind::Unexpected,
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
    pub(crate) fn lex_atom(&self, start_pos: usize) -> &'a str {
        let mut end_pos = start_pos + 1;

        while let Some(&b) = self.input.get(end_pos) {
            if is_atom_tail(b) {
                end_pos += 1;
            } else {
                break;
            }
        }

        let bytes = self.input.get(start_pos..end_pos).unwrap();
        std::str::from_utf8(bytes).unwrap()
    }

    pub(crate) fn lex_int(&self, pos: usize) -> lex::Result<(i32, usize)> {
        let mut cursor = pos;
        let mut num: i32 = 0;
        while let Some(&b) = self.input.get(cursor) {
            match b {
                b'0'..=b'9' => {
                    num = num
                        .checked_mul(10)
                        .and_then(|num| num.checked_add(i32::from(b - b'0')))
                        .ok_or(lex::Error {
                            pos,
                            kind: lex::error::Kind::Overflow,
                        })?;

                    cursor += 1;
                }
                _ if is_atom_tail(b) => {
                    return Err(lex::Error {
                        pos: cursor,
                        kind: lex::error::Kind::BadDigit,
                    });
                }
                _ => break,
            }
        }

        Ok((num, cursor))
    }

    // `pos` points to the left quotation mark '"'
    pub(crate) fn lex_str(&self, pos: usize) -> lex::Result<&str> {
        let len = self.input[pos + 1..]
            .iter()
            .position(|b| *b == b'"')
            .ok_or(lex::Error {
                pos,
                kind: lex::error::Kind::Quote,
            })?;

        let s = &self.input[pos + 1..][..len];
        let s = std::str::from_utf8(s).unwrap();

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

fn to_keyword(atom: &str) -> Option<Kind> {
    match atom {
        "let" => Some(Kind::Let),
        "true" => Some(Kind::True),
        "false" => Some(Kind::False),
        "fn" => Some(Kind::Fn),
        "if" => Some(Kind::If),
        "else" => Some(Kind::Else),
        "return" => Some(Kind::Return),
        _ => None,
    }
}
