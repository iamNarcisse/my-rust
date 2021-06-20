use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

pub fn index() {
    let my_str = "hello";
    let my_string = String::from(my_str);

    let int = 5;

    // let num = Number::from(30);

    let mum: Number = int.into();

    println!("My number is {:?}", mum);

    println!("============> {}", my_string);
}
