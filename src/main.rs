use jtheorems::{
    judgement::Judgement,
    terms::{primitives::NaturalType, variable::VariableData, Term, TermData},
};

fn main() {
    //let vardata = VariableData::new("x", Term::new());
    println!("{}", std::mem::size_of::<Box<str>>());
    println!("{}", std::mem::size_of::<VariableData>());
    println!("{}", std::mem::size_of::<TermData>());

    let genesis = Judgement::empty();
    println!("{}", &genesis);
    let nat_form = genesis.natural_formation().unwrap();
    println!("{}", &nat_form);
}
