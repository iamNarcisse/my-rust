// use casting::index;

// #[path = "vectors/vectors.rs"]

#[path = "download/download.rs"]
mod download;
#[path = "variable/mutability.rs"]
mod mutability;
#[path = "variable/variable.rs"]
mod variable;

#[path = "variable/scoping.rs"]
mod scoping;

#[path = "types/casting.rs"]
mod casting;

#[path = "inference/inference.rs"]
mod inference;

#[path = "types/aliasing.rs"]
mod aliasing;

fn main() {
    // vectors::vectors();

    // variable::index();

    casting::index();
    mutability::index();
    scoping::index();
    inference::index();
    aliasing::index();
    println!("Hello, world!");
}
