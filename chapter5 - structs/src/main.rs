
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}




fn main() {
    let mut user1 = User {
        email: String::from("example@mail.com"),
        username: String::from("example123"),
        active: true,
        sign_in_count: 1
    };

    let name = user1.username;
    user1.username = String::from("nightly123");

    let user2 = build_user(String::from("kyle@mail.com"),
                           String::from("kyle123"));

    let user3 = User {
        email: String::from("james@mail.com"),
        username: String:: from("james123"),
        ..user2
    };

    struct Color(i32, i32, i32);

    struct Point(i32, i32, i32);

    let width = 30;
    let height = 50;
    print_area(area0(width, height));
    print_area(area1((30, 50)));

    let rect = Rectangle {
        width: 30,
        height: 50
    };

    println!("{:#?}", rect);
    print_area(area2(&rect));
    print_area(rect.area());

    let rect1 = Rectangle {
        width: 20,
        height: 40
    };

    let rect2 = Rectangle {
        width: 40,
        height: 50
    };

    let rect3 = Rectangle::square(25);

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));
    println!("rect can hold rect3: {}", rect.can_hold(&rect3));


}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

fn print_area(area: u32){
    println!("The area of the rectangle is {} square pixels.", area);
}

fn area0(width: u32, height: u32) -> u32 {
    width * height
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}