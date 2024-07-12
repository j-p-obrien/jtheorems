use jtheorems::{deduction::terminal::Terminal, parser::parser::Parser};

fn main() {
    let mut _terminal = Terminal::new();
    let parser = Parser::new();
    loop {
        println!("{}", &_terminal);
        let mut raw_input = String::new();
        std::io::stdin().read_line(&mut raw_input).unwrap();
        let _parsed_input = parser.parse(&raw_input);
    }
}
