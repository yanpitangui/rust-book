use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

use std::rc::Rc;
enum List {
    Cons(i32, Rc<List>),
    Nil
}

use crate::List::{Cons, Nil};

fn main() {
    println!("Hello, world!");

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *(y.deref()));

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    let c = CustomSmartPointer {
        data: String::from("some data"),
    };

    drop(c);

    println!("CustomSmartPointer dropped before the end of main.");

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));

    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}