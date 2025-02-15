use jtheorems::terminal::terminal::Terminal;

fn main() {
    let mut terminal = Terminal::new();
    let identity_function_signature = "def identity : {A : Type} -> A -> A :=";
    terminal.parse(&identity_function_signature);
}
