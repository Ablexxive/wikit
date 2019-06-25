use std::error::Error;
use std::process;

extern crate wikipedia;
//TODO - add a random option :) - https://seppo0010.github.io/wikipedia-rs/wikipedia/struct.Wikipedia.html#method.random


pub fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let term = args[1].to_owned();
    let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();

    // TODO - if you want, write update wikipedia-rs to have an error Trait to allow use of ?
    //let page = wiki.search(&term)?;
    let search_results = wiki.search(&term).unwrap_or_else(|err| {
        println!("Error: {}", err);
        process::exit(1);
    });

    let page = wiki.page_from_title(search_results[0].to_owned());
    let content = page.get_summary().unwrap();
    println!("{}", content);
    Ok(())
}
