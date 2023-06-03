#[derive(Debug)]
pub enum Token {
    EOF,
    EOL,
    Pipe,
    Assign,
    Gt,
    Lt,
    Semicolon,
    Dollar,
    Ampersand,
    Include,
    Ident(String),
    String(String),
    Fd(usize),
}
