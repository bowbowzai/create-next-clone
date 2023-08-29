fn main() {
    match create_next_clone::run() {
        Ok(_) => (),
        Err(_) => println!("Something went wrong!!!"),
    };
}
