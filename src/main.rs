use std::process;


fn main() {
    let term = wikit::parse_input();
    if let Err(e) = wikit::run(&term) {
        println!("Application error: {}", e);
        process::exit(1);
    }

}
