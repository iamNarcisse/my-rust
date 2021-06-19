pub fn index() {
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;
        println!("Inner short: {}", short_lived_binding);
    }

    // println!("outer short : {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);
}
