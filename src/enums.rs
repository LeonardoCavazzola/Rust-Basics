enum Curr {
    BRL,
    USD,
    EUR,
    Other { acron: String, from: String },
}

pub(crate) fn run() {
    let cur = Curr::Other { acron: String::from("JPY"), from: String::from("Japan") };

    match cur {
        Curr::BRL => println!("brazilian currency"),
        Curr::USD => println!("american currency"),
        Curr::EUR => println!("european currency"),
        Curr::Other { acron, from } => println!("currency {}, from {}", acron, from),
    }
}
