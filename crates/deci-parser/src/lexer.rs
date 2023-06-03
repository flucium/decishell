use crate::token::Token;

pub struct Lexer {
    source: Vec<char>,
    position: usize,
    peek: Option<Token>,

    is_eof: bool,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
            peek: None,
            is_eof: false,
        }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        if self.peek.is_none() {
            self.peek = self.read();
        }

        self.peek.as_ref()
    }

    fn read(&mut self) -> Option<Token> {
        while let Some(ch) = self.source.get(self.position) {
            self.position += 1;

            if ch.is_whitespace() {
                continue;
            }

            match ch {
                '#' => {
                    while let Some(ch) = self.source.get(self.position) {
                        self.position += 1;
                        if ch == &'\n' {
                            break;
                        }
                    }
                }

                // ...
                '\n' => {
                    return Some(Token::EOL);
                }

                '|' => {
                    return Some(Token::Pipe);
                }

                ';' => {
                    return Some(Token::Semicolon);
                }

                '>' => {
                    return Some(Token::Gt);
                }

                '<' => {
                    return Some(Token::Lt);
                }

                '$' => {
                    return Some(Token::Dollar);
                }

                '&' => {
                    return Some(Token::Ampersand);
                }

                // ...
                '"' => {
                    if let Some(string) = self.read_ws_esc_string() {
                        return Some(Token::String(string));
                    }
                }

                _ => {
                    if let Some(string) = self.read_string() {
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

    fn read_string(&mut self) -> Option<String> {
        let mut buffer = String::new();

        // Get current char.
        // self.position is +=1. Therefore, to know the current pos, do self.position - 1.
        buffer.push(*self.source.get(self.position - 1)?);

        while let Some(ch) = self.source.get(self.position) {
            if ch.is_whitespace() || matches!(ch, ';' | '=' | '|' | '>' | '<') {
                break;
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

    fn read_ws_esc_string(&mut self) -> Option<String> {
        let mut buffer = String::new();

        while let Some(ch) = self.source.get(self.position) {
            if ch == &'"' {
                self.position += 1;
                break;
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
