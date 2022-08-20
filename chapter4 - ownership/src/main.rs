fn main() {
    let s = String::from("hello world");
    takes_ownership(s);

    // this is not possible, because passing a string to 
    // a function moves its chapter4 - ownership
    // println!("{}", s);

    let x = 5;
    makes_copy(x);
    // this is possible because integers are copied.
    println!("{}", x);

    let s1 = gives_ownership();
    println!("s1 = {}", s1);

    let s2 = String::from("hello");
    let len = calculate_length(&s2);
    println!("The lenth of '{}' is {}.", s2, len);

    let mut mutable = String::from("mutate me");
    change(&mut mutable);
    println!("{}", mutable);

    let to_be_sliced = String::from("hello world");
    let hello = &to_be_sliced[..5];
    let world = &to_be_sliced[..];

    println!("{}, {}", hello, world);


    let slice = "hello world";
    let word = first_word(&to_be_sliced);
    let word2 = first_word(&slice);

    println!("{}, {}", word, word2);

    let a = [1,2,3,4,5];
    let slice = &a[0..2];
    for i in slice.iter() {
        println!("{}", i);
    }
}


fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

// returning a string gives chapter4 - ownership
fn gives_ownership() -> String {
    let some_string = String::from("_");
    some_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(" please");
}

fn first_word(s: &str) ->  &str {
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
     
}