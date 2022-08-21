fn main() {

    let largest = get_largest(vec![9, 5, 2, 10, 29]);
    println!("The largest number is {}", largest);

    let largest = get_largest(vec!['a', 'b', 'z']);
    println!("The largest number is {}", largest);

    let p1 = Point { x: 5, y: 10};
    let p2 = Point { x: 5.0, y: 10.0};

    p1.x();
    p2.y();
    let p3 = Point { x: "Hello", y: 'c' };

    let p4 = p1.mixup(p3);

    println!("p4.x = {}, p4.y = {}", p4.x, p4.y);



}

fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

struct Point<T, U> {
    x: T,
    y: U
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

impl<T> Point<T, f64> {
    fn y(&self) -> &f64 {
        &self.y
    }

}