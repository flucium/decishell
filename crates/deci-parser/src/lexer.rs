use crate::token::Token;
use std::fs::File;
use std::io::Read;

pub struct Lexer {
    source: Vec<char>,
    source_length: usize,
    position: usize,
    is_eof: bool,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            source_length: source.len(),
            position: 0,
            is_eof: false,
        }
    }

    pub fn new_from_str(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            source_length: source.len(),
            position: 0,
            is_eof: false,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            tokens.push(self.read());
            if tokens.last().unwrap_or(&Token::EOF) == &Token::EOF {
                break;
            }
        }

        tokens
    }

    fn read(&mut self) -> Token {
        let ch = loop {
            let ch = match self.next_ch() {
                None => return Token::EOF,
                Some(ch) => ch,
            };

            if ch == '\n' || ch.is_whitespace() == false {
                break ch;
            }
        };

        match ch {
            '#' => {
                //skip commentout
                while let Some(ch) = self.next_ch() {
                    if ch == '\r' || ch == '\n' {
                        break;
                    }
                }

                self.read()
            }

            '&' => {
                if let Some(peek_ch) = self.peek_ch() {
                    if peek_ch.is_whitespace() == false {
                        let string = self.next_string(None, false).unwrap_or_default();

                        if let Ok(n) = string.parse::<usize>() {
                            return Token::Fd(n);
                        }
                    }
                }

                Token::Ampersand
            }

            '$' => {
                if let Some(peek_ch) = self.peek_ch() {
                    if peek_ch.is_whitespace() == false {
                        return self
                            .next_string(None, false)
                            .map(|string| Token::Ident(string))
                            .unwrap_or(Token::Dollar);
                    }
                }

                Token::Dollar
            }

            '|' => Token::Pipe,

            '=' => Token::Assign,

            '>' => Token::Gt,

            '<' => Token::Lt,

            // \r || \n
            '\n' => Token::EOL,

            ';' => Token::Semicolon,
            
            'i' | 'I' => {
                let string = self.next_string(Some(ch), false).unwrap_or_default();

                if string.to_lowercase() == "include" {
                    Token::Include
                } else {
                    Token::String(string)
                }
            }

            // 'hello world'
            '\'' => self
                .next_string(None, true)
                .map(|string| Token::String(string))
                .unwrap_or(Token::EOF),

            // "hello world"
            '"' => self
                .next_string(None, true)
                .map(|string| Token::String(string))
                .unwrap_or(Token::EOF),

            // hello
            _ => self
                .next_string(Some(ch), false)
                .map(|string| Token::String(string))
                .unwrap_or(Token::EOF),
        }
    }

    // ...
    fn next_string(&mut self, current_char: Option<char>, whitespace: bool) -> Option<String> {
        let mut buffer = match current_char {
            None => String::new(),
            Some(ch) => String::from(ch),
        };

        while let Some(ch) = self.source.get(self.position) {
            if ch.is_whitespace() && whitespace == false
                || matches!(ch, ';' | '=' | '|' | '>' | '<' | '\n')
            {
                break;
            }

            if ch == &'"' || ch == &'\'' && whitespace == true {
                self.position += 1;
                break;
            }

            buffer.push(*ch);
            self.position += 1;
        }

        match buffer.is_empty() {
            true => None,
            false => Some(buffer),
        }
    }

    fn peek_ch(&self) -> Option<&char> {
        self.source.get(self.position)
    }

    fn next_ch(&mut self) -> Option<char> {
        if self.position > self.source_length {
            return None;
        }

        let ch = self.source.get(self.position)?;

        self.position += 1;

        Some(*ch)
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.read();

        if token == Token::EOF {
            if self.is_eof {
                None?
            } else {
                self.is_eof = true;
            }
        }

        Some(token)
    }
}

impl From<File> for Lexer {
    fn from(mut file: File) -> Self {
        let mut buffer = String::new();

        file.read_to_string(&mut buffer).unwrap();

        Self {
            source: buffer.chars().collect(),
            source_length: buffer.len(),
            position: 0,
            is_eof: false,
        }
    }
}

impl From<&File> for Lexer {
    fn from(mut file: &File) -> Self {
        let mut buffer = String::new();

        file.read_to_string(&mut buffer).unwrap();

        Self {
            source: buffer.chars().collect(),
            source_length: buffer.len(),
            position: 0,
            is_eof: false,
        }
    }
}
