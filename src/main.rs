use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

pub mod lexer;
pub mod parser;
pub mod interpreter;

fn main() {
    let input = r#"
    x = 0
    while x < 10 do
        print x
        x = x + 1
    end
    "#;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();

    let mut interpreter = Interpreter::new();
    interpreter.interpret(ast);
}