use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    println!("{}", "Call To helloworld()") ;
    hello_world();

  let username: Result<String, io::Error> =   read_username_from_file();
  println!("{}", username.unwrap())  ;
}

fn hello_world(){
    println!("Hello, world!");
}


fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;

    // // https://blog.rust-lang.org/2016/11/10/Rust-1.13.html
    // // Comment to using '?' operator from 1.13 Version
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)
    // // Comment to using '?' operator from 1.13 Version
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
}
