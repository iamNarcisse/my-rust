type Narcisse = u64;
type Inch = u64;

// Use an attribute to silence warning.
#[allow(non_camel_case_types)]
// type u64_t = u64;

pub fn index() {
    let narcisse: Narcisse = 5;

    let inches: Inch = 2;

    println!(
        "{} narcisse + {} inches = {} unit?",
        narcisse,
        inches,
        narcisse + inches
    )
}
