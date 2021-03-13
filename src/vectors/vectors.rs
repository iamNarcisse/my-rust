use std::vec;

//Vectors can only store values of the same type

pub fn vectors() {
    //METHOD 1.
    // let mut v = Vec::new();

    // v.push(5);
    // v.push(6);
    // v.push(6);

    //METHOD 2.
    let v = vec![1, 2, 5, 6, 4];

    let third: &i32 = &v[2];

    println!("Hello there!{:?}", v);
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    //Panic

    let my_vec = vec![1, 2, 3, 4, 5, 6];
    // let does_not_exist = &my_vec[100];
    let does_not_exist = my_vec.get(100);

    if does_not_exist == None {
        println!("{:?}, It does not exist ", does_not_exist);
    }

    println!("{:?}", does_not_exist);

    //ITERATING A VECTOR

    let v = vec![100, 32, 57];

    for i in v {
        println!("{}", i + 1);
    }

    //INTERATING A MUTALBE VECTOR

    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;

        println!("{}", i);
    }

    // STORING ENUM TO STORE MULTIPLE TYPES

    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];
}
