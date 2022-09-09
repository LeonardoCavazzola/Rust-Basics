fn run() {
    match result_err() {
        Ok(it) => println!("All ok: {}", it),
        Err(it) => println!("All wrong: {}", it),
    }
}

fn throw_an_error() {
    panic!("ola");
}

fn result_ok() -> Result<String, String> {
    return Ok(String::from("ok"));
}

fn result_err() -> Result<String, String> {
    return Err(String::from("Error lalalala"));
}
