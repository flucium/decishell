use crate::token::Token;

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,

    peek: Option<Token>,

    position: usize,

    is_eof: bool,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            peek: None,
            position: 0,
            is_eof: false,
        }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        if self.peek.is_none() {
            self.peek = self.read();
        }

        self.peek.as_ref()
    }

    fn consume(&mut self) {}

    fn skip_commentout(&mut self) {
        if matches!(self.source.get(self.position), Some('#')) == false {
            return;
        }

        while let Some(ch) = self.source.get(self.position) {
            self.position += 1;
            if ch == &'\n' {
                break;
            }
        }
    }

    fn read(&mut self) -> Option<Token> {
        while let Some(ch) = self.source.get(self.position) {
            //
            if ch.is_whitespace() {
                self.position += 1;
                continue;
            }

            match ch {
                // ...
                '#' => {
                    // is include file
                    if let Some(ch) = self.source.get(self.position + 1) {
                        if ch == &'i' || ch == &'I' {
                            let current_pos = self.position;
                            self.position += 1;

                            if self
                                .read_string(false)
                                .and_then(|string| Some(string.to_lowercase()))
                                == Some(String::from("include"))
                            {
                                return Some(Token::Include);
                            }

                            self.position = current_pos;
                        }
                    }

                    // is comment out
                    self.skip_commentout();
                }

                // ...
                '\n' => {
                    self.position += 1;
                    return Some(Token::EOL);
                }

                '|' => {
                    self.position += 1;
                    return Some(Token::Pipe);
                }

                ';' => {
                    self.position += 1;
                    return Some(Token::Semicolon);
                }

                '>' => {
                    self.position += 1;
                    return Some(Token::Gt);
                }

                '<' => {
                    self.position += 1;
                    return Some(Token::Lt);
                }

                '$' => {
                    self.position += 1;
                    return Some(Token::Dollar);
                }

                '&' => {
                    self.position += 1;
                    return Some(Token::Ampersand);
                }

                // ...
                '"' => {
                    self.position += 1;

                    if let Some(string) = self.read_string(true) {
                        return Some(Token::String(string));
                    }
                }

                _ => {
                    if let Some(string) = self.read_string(false) {
                        return Some(Token::String(string));
                    }
                }
            }
        }

        if self.is_eof {
            None
        } else {
            self.is_eof = true;
            Some(Token::EOF)
        }
    }
    

    fn read_string(&mut self, is_escape: bool) -> Option<String> {
        let mut buffer = String::new();

        while let Some(ch) = self.source.get(self.position) {
            if is_escape {
                if ch == &'"' {
                    self.position += 1;
                    break;
                }
            } else {
                if ch.is_whitespace() || matches!(ch, ';' | '=' | '|' | '>' | '<') {
                    break;
                }
            }

            self.position += 1;

            buffer.push(*ch);
        }

        if buffer.is_empty() {
            None
        } else {
            Some(buffer)
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.peek.take() {
            return Some(token);
        }

        self.read()
    }
}
