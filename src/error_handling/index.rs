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
pub fn main() {
    // drink("beer");
    // drink("wine");

    let water = Some("Narcise");
    let lemonade = Some("lemonade");
    give_adult(water)
}
