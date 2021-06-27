use core::panic;

fn drink(drink: &str) {
    if drink == "beer" {
        panic!("Aaaaaaaaaah!!!");
    }

    println!("Some refreshing {} is all I need", drink);
}

fn give_adult(drink: Option<&str>) {
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary"),
        Some(inner) => println!("{}? How nice", inner),
        None => println!("Nothing provided"),
    }
}

fn drink_with_unwrap(drink: Option<&str>) {
    let data = drink.unwrap();

    if data == "lemonade" {
        panic!("Oh no!, not lemonade again!")
    }

    println!("{}, How nice", data);
}

fn next_birthday(current_age: Option<u8>) -> Option<String> {
    let next_age: u8 = current_age?;

    Some(format!("Next year I will be {}", next_age))
}

struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    fn work_phone_area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }
}

fn test() {
    let person = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(21),
                number: 121,
            }),
        }),
    };

    assert_eq!(person.work_phone_area_code(), Some(21))
}

pub fn main() {
    // drink("beer");
    // drink("wine");

    // let water = Some("Narcise");
    // let lemonade = Some("lemonade");
    // give_adult(water);
    // give_adult(lemonade);
    // drink_with_unwrap(Some("lemonade"));

    let dob = next_birthday(Some(21));

    println!("{:?} ", dob)
}
