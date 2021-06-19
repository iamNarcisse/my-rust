fn shadowing() {
    let shadowed_binding = 1;
    {
        println!("before being shadowed: {}", shadowed_binding);

        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }

    println!("outside inner block:{}", shadowed_binding);

    let shadowed_binding = 2;

    println!("shadowed in outer block: {}", shadowed_binding);
}

fn freezing() {
    let mut _to_be_freezed = 1;

    {
        let _to_be_freezed = _to_be_freezed;
    }

    _to_be_freezed = 10;
}
pub fn index() {
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;
        println!("Inner short: {}", short_lived_binding);
    }

    // println!("outer short : {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);

    shadowing();

    freezing();
}
