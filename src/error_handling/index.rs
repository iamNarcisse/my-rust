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

pub fn main() {
    // drink("beer");
    // drink("wine");

    let water = Some("Narcise");
    // let lemonade = Some("lemonade");
    give_adult(water);
    // give_adult(lemonade);
    drink_with_unwrap(Some("lemonade"))
}
