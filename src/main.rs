use deci_parser::*;

fn main() {
    let mut lex = lexer::Lexer::new("#include hello.sh #hello\nls -a".to_string());
    println!("{:?}", lex.next());
    println!("{:?}", lex.next());
    println!("{:?}", lex.next());
    println!("{:?}", lex.next());
    println!("{:?}", lex.next());
    println!("{:?}", lex.next());
}
