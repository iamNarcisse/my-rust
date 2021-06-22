#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct

struct Unit;

// A tuple struct
#[derive(Debug)]
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

pub fn index() {
    let name = String::from("Peter");
    let age = 27;

    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 2.4 };

    println!("Point coordinates: ({}, {})", point.x, point.y);

    let bottom_right: Point = Point { x: 5.6, ..point };

    println!("Bottom right {:?}", bottom_right);

    let Point {
        x: top_edge,
        y: left_edge,
    } = point;

    let _rectange = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right,
    };

    let pair = Pair(1, 3.9);

    println!("ACESS ME  -> {:?}", pair);

    let Pair(integer, decimal) = pair;

    println!("=======>{} ====> {}", integer, decimal);

    let result = square(point, 10.0, 5.0);

    println!("{:?}", result);

    let area = rect_area(_rectange);

    println!("AREA =====>{:?}", area);
}

fn square(point: Point, top: f32, bottom: f32) -> Rectangle {
    let result: Rectangle = Rectangle {
        top_left: Point { ..point },
        bottom_right: Point { y: top, x: bottom },
    };

    result
}

fn rect_area(rectange: Rectangle) -> f32 {
    // l * b
    let Rectangle {
        top_left: Point { x, y },
        bottom_right: Point { x: x1, y: y1 },
    } = rectange;

    let width = x1 - x;
    let height = y1 - y;

    height * width
}
