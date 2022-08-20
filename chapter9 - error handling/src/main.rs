extern crate core;

use std::error::Error;
use std::fs::{self, File};
use std::io;
use std::io::{ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
    a();
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    };

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            // this is a expression
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // this can only be used on functions that return values
    let f = File::open("hello.txt")?;

    Ok(())

}

fn read_username_from_file() -> Result<String, io::Error> {
    /*
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)*/

    fs::read_to_string("hello.txt")

    /*
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };*/



    /*
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }*/
}

fn a() {
    b();
}

fn b() {
    c(21);
}

fn c(num: i32) {
    if num == 22 {
        panic!("Don't pass in 22!");
    }
}