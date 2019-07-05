use std::error::Error;
use std::process;

extern crate wikipedia;
extern crate clap;
use clap::{Arg, App};


pub fn parse_input() -> String {
    let matches = App::new("Wikipedia CLI helper.")
                          .author("Sahil K. <sahilsan@gmail.com>")
                          .about("Prints out summary of desired wikipedia page.")
                          .arg(Arg::with_name("SEARCH_TERM")
                               .help("Search term for wikipedia page. If none is provided, a random article is chosen.")
                               .required(false)
                               .index(1))
                          .get_matches();
    matches.value_of("SEARCH_TERM").unwrap_or("get_random_article").to_owned()
}


pub fn run(term: &str) -> Result<(), Box<dyn Error>> {
    let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();

    let search_term = match term {
        "get_random_article" => wiki.random().unwrap().unwrap(),
        _ => term.to_string(),
    };

    println!("Search Term: {}", search_term);
    let search_results = wiki.search(&search_term).unwrap_or_else(|err| {
        println!("Error: {}", err);
        process::exit(1);
    });

    let page = wiki.page_from_title(search_results[0].to_owned());
    let content = page.get_summary().unwrap();
    println!("{}", content);
    Ok(())
}
