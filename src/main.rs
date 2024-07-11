use jtheorems::{judgement::Deduction, parser::parser::Parser};

fn main() {
    let mut deduction = Deduction::new();
    let parser = Parser::new();
    loop {
        println!("{}", &deduction);
        let mut raw_input = String::new();
        std::io::stdin().read_line(&mut raw_input).unwrap();
        let parsed_input = parser.parse(&raw_input);
    }
}
