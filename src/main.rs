extern crate curl;
extern crate clap;

use std::io::{stdout, Write};
use clap::{Arg, App};

use curl::easy::Easy;

fn perform_request(host: &str) {
    let mut easy = Easy::new();
    easy.url(host).unwrap();
    easy.write_function(|data| {
        Ok(stdout().write(data).unwrap())
    }).unwrap();
    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());
}


fn main() {
    let matches = App::new("My utilite for osm")
                        .version("1.0")
                        .author("Ivan Lavriv: <lavriv92@gmail.com>")
                        .about("Simple utilite for osm")
                        .arg(Arg::with_name("host")
                             .short("d")
                             .takes_value(true)
                             .long("host")
                             .help("Sets custom osm host"))
                        .get_matches();

    let host = matches.value_of("host").unwrap_or("https://www.rust-lang.org/");
    println!("{}", host);
    perform_request(host);
}
