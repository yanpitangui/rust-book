use std::fmt;
use std::fmt::{Formatter, write};

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

struct Age(u32);

struct ID(u32);

fn main() {
    let w = Wrapper(
        vec![String::from("hello"), String::from("world")]
    );

    println!("w = {}", w);

    // type alias
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {}

    fn returns_long_type() -> Thunk {
        Box::new(|| println!("Iha"))
    }

    let s1 = "Hello there!";
    let s2 = "How's it going?";
}