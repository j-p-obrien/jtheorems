use jtheorems::{parse::parser::Parser, terminal::terminal::Terminal};

fn main() {
    let identity_function_signature = "def identity{A : U}(x:a) : A :=";
    let mut parser = Parser::new(&identity_function_signature);
    parser.inc();
    let mut terminal = Terminal::new();
}
