fn run() {
    switch_case();
}

fn if_else() {
    let number: u8 = 1;
    if number < 2 {
        println!("it is lower two");
    } else {
        println!("it is equals or higher two");
    }
}

fn ternary() {
    let number: u8 = 1;
    let result = if number < 2 {
        "it is lower two"
    } else {
        "it is equals or higher two"
    };
    println!("{}", result);
}

fn switch_case() {
    let number: u8 = 12;

    match number {
        1 | 3 | 5 | 7 | 9 => println!("Odd"),
        2 | 4 | 6 | 8 => println!("Even"),
        10..=20 => println!("From ten to twenty"),
        _ => println!("Other"),
    }
}
