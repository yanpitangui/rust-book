
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice<T>(f: T, arg: i32) -> i32
where T: Fn(i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1,2,3];
    let list_of_strings: Vec<String> =
        list_of_numbers
            .iter()
            //.map(|i| i.to_string())
            // OR
            .map(ToString::to_string)
            .collect();

    println!("{:?}", list_of_strings)

    enum Status {
        Value(u32),
        Stop
    }

    let list_of_statuses: Vec<Status> =
        (0u32..20).map(Status::Value).collect();
}

fn returns_closure(a: i32) -> fn(i32) -> i32 {
    if a > 0 {
        | b | b + 1
    } else {
        | b | b - 1
    }
}
