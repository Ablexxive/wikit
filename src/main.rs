use std::env;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Need to provide a search term :)");
        process::exit(1);
    }

    if let Err(e) = wikit::run(args) {
        println!("Application error: {}", e);
        process::exit(1);
    }

}
