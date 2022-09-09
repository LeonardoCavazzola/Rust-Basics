pub(crate) fn array() {
    let a_array: [f32; 3] = [9.5, 5.0, 6.4];
    let index: usize = 0;
    println!("by index {}", a_array[index]);
    println!("by zero {}", a_array[0]);
    println!("=================================================");

    for value in a_array {
        println!("{}", value);
    }

    println!("=================================================");
    println!("length = {}", a_array.len());
}

pub(crate) fn matriz() {
    let a_matriz: [[f64; 2]; 2] = [
        [9.5, 5.0],
        [9.5, 5.0],
    ];
    println!("{}", a_matriz[0][0]);
    println!("=================================================");

    for lin in a_matriz {
        for value in lin {
            println!("{}", value);
        }
    }

    println!("=================================================");
    println!("length = {}", a_matriz.len());
}
