use jtheorems::judgement::Judgement;

fn main() {
    let genesis = Judgement::empty();
    println!("{}", &genesis);
    let nat_form = genesis.natural_formation().unwrap();
    println!("{}", &nat_form);
}
