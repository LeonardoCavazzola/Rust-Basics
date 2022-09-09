fn run() {
    let mut mutable_pointer_to_string = String::from("mut str");
    let pointer_to_string = String::from("str");

    println!("{}", mutable_pointer_to_string);
    receive_a_mutable_ref(&mut mutable_pointer_to_string);
    println!("{}", mutable_pointer_to_string);

    receive_an_immutable_ref(&pointer_to_string);
}

fn receive_a_mutable_ref(string: &mut String) {
    string.push_str(" dif");
    println!("{}", string);
}

fn receive_an_immutable_ref(string: &String) {
    println!("{}", string);
}
