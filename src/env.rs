extern crate dotenv;

use dotenv::dotenv;
use std::env;

fn main() {
    match env::var("LANG") {
        Ok(lang) => println!("Language code: {}", lang),
        Err(e) => println!("Couldn't read LANG ({})", e),
    };

   dotenv().ok();

    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
}

