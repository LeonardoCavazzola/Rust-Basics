pub(crate) fn run() {
    match some() {
        Some(it) => { println!("{}", it); }
        None => { println!("Not Found") }
    }

    println!("{:?}", none())
}


fn some() -> Option<String> {
    Some(String::from("Leo"))
}

fn none() -> Option<String> {
    None
}
