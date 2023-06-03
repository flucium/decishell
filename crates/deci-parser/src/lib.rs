pub mod lexer;
pub mod token;
use std::fs::File;

use self::lexer::Lexer;
pub struct Parser {}

impl Parser {}

impl From<File> for Parser{
    fn from(file: File) -> Self {        
        todo!()
    }
}

pub fn parse() {}

fn parse_command() {}

fn parse_command_program() {}

fn parse_command_args() {}

fn parse_redirect() {}

fn parse_assign() {}

fn parse_variable() {}

fn parse_fd() {}

fn parse_string(lexer: &Lexer) {}

pub enum Operator {
    // Equal,
    // NotEqual,
    Gt,
    Lt,
}

pub enum Node {
    Redirect {
        operator: Operator,
        left: Box<Node>,
        right: Box<Node>,
    },

    Command {
        program: Box<Node>,
        args: Box<Node>,
    },

    List(Vec<Node>),

    Assign {
        ident: Box<Node>,
        value: Box<Node>,
    },

    String(String),

    Fd(usize),
}