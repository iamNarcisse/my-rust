fn handle_while() {
    let mut n = 1;

    while n < 101 {
        if n % 15 == 0 {
            println!("FizzBuzz {}", n)
        } else if n % 5 == 0 {
            println!("Buzz {}", n)
        } else if n % 3 == 0 {
            println!("Fizz {}", n)
        } else {
            println!("{}", n)
        }

        n += 1;
    }
}

fn for_range() {
    for n in 1..101 {
        if n % 15 == 0 {
            println!("FizzBuzz {}", n)
        } else if n % 5 == 0 {
            println!("Buzz {}", n)
        } else if n % 3 == 0 {
            println!("Fizz {}", n)
        } else {
            println!("{}", n)
        }
    }
}

fn iterators() {
    let mut names = vec!["Narcisse", "John"];

    // for name in names.into_iter() {
    //     println!("Name===> {}", name);

    //     match name {
    //         "Narcisse" => println!("Imela Odogwu {}", name),
    //         _ => println!("Hello {}", name),
    //     }
    // }
    // for name in names.iter() {
    //     println!("{} =====>", &name);
    // }

    for name in names.iter_mut() {
        *name = match name {
            &mut "Narcisse" => "This is a ninja",
            _ => "hELLO",
        }
    }

    println!("====> {:?} ", names);
}

fn handle_match() {
    let number = 13;

    println!("Tell me about {} ", number);

    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 => println!("You are a prime"),
        13..=19 => println!("Teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;

    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} =====> {}", boolean, binary);
}

fn match_tuple() {
    let my_numbers = (0, 3, -8);
    println!("Tell me about {:?}", my_numbers);

    match my_numbers {
        (0, z, y) => println!("First is 0, y is {:?} and z is {:?}", z, y),

        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        _ => println!("Nah!"),
    }
}

pub fn index() {
    handle_while();
    for_range();
    iterators();
    handle_match();
    match_tuple();
}
