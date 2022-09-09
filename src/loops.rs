fn run() {
    forr();
}

fn whilee() {
    let multiplicand: u8 = 5;
    let mut multiplier: u8 = 0;

    while multiplier <= 10 {
        println!(
            "{} * {} = {}",
            multiplicand,
            multiplier,
            multiplicand * multiplier
        );
        multiplier += 1;
    }
}

fn infinite() {
    let multiplicand: u8 = 5;
    let mut multiplier: u8 = 0;

    loop {
        println!(
            "{} * {} = {}",
            multiplicand,
            multiplier,
            multiplicand * multiplier
        );

        multiplier += 1;

        if multiplier > 10 {
            break;
        };
    }
}

fn forr() {
    let multiplicand: u8 = 5;
    for multiplier in 0..=10 {
        println!(
            "{} * {} = {}",
            multiplicand,
            multiplier,
            multiplicand * multiplier
        );
    }
}

