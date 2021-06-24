use core::f64;

// implement FizzBuzz

fn is_divisible(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}

fn fizz_buzz(num: u32) {
    if is_divisible(num, 15) {
        println!("====> FizzBuzz {}", num)
    } else if is_divisible(num, 5) {
        println!("====> Buzz {}", num)
    } else if is_divisible(num, 3) {
        println!("====> Fizz {}", num)
    } else {
        println!("====> Nada! {}", num)
    }
}

struct Person {
    name: String,
    age: i32,
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new_point(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}
struct Rectangle {
    p1: Point,
    p2: Point,
}

struct Circle {
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        // pi * r * 2
        (3.132) * self.radius.exp2()
    }

    fn perimeter(&self) -> f64 {
        // 2 * pi * r

        2.0 * 3.142 * self.radius
    }
}

impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }
}

fn closures() {
    fn function(i: i32) -> i32 {
        i + 1
    }

    let closure_annotated = |i: i32| -> i32 { i + 1 };

    let closure_inferred = |i: i32| i + 1;

    let i = 2;

    println!("function: {}", function(i));
    println!("closure annotated: {}", closure_annotated(i));
    println!("closure inferred: {}", closure_inferred(i));

    let one = || 1;

    println!("closure returning one: {}", one())
}
pub fn main() {
    fizz_buzz(12);

    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new_point(2.0, 4.0),
    };

    let circle: Circle = Circle { radius: 4.0 };

    println!("Area of a circle {} ", circle.area());

    println!("Perimeter of a circle {} ", circle.perimeter());

    println!("=======> {}", rectangle.perimeter());
    println!("Area of rectangle =======> {}", rectangle.area());

    closures();
}
