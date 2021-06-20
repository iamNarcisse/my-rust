pub fn index() {
    let elem = 5u8;

    let mut vec = Vec::new();

    println!("Here I am, a vector-> {:?}", vec);

    vec.push(elem);

    println!("{:?}", vec);
    println!("{:?}", (vec).len());
}
