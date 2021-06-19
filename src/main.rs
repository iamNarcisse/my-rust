// #[path = "vectors/vectors.rs"]

#[path = "download/download.rs"]
mod download;
#[path = "variable/mutability.rs"]
mod mutability;
#[path = "variable/variable.rs"]
mod variable;

#[path = "variable/scoping.rs"]
mod scoping;

fn main() {
    // vectors::vectors();

    // variable::index();
    mutability::index();
    scoping::index();
    println!("Hello, world!");
}
