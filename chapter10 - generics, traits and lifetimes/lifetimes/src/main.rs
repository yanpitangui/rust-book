use std::fmt::Display;

fn main() {
    let r;

    {
        let x = 5;
        r = x;
    }

    println!("r: {}", r);

    let x = 5;
    let r = &x;
    println!("r: {}", r);

    let string1 = String::from("abcd");
    let mut result;
    {
        let string2 = String::from("xyz");

        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
        result = longest_x(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);


    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next()
        .expect("Could not find first sentence");

    let i = ImportantExcerpt {
        part: first_sentence
    };
    let s: &'static str = "I have a static lifetime";



}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn return_part(&self, announcement: &str) -> &str{
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_x<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// the compiler can deterministically infer the lifetime annotation
// using lifetime elision
// 1. Each parameter that is a reference gets its own lifetime parameter
// 2. If there is exactly one input lifetime parameter, that lifetime is
// assigned to all output lifetime parameters.
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self
// the lifetime of self is assigned to all output lifetime parameters.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..1];
        }
    }

    &s[..]
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}