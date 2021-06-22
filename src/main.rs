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

#[path = "conversion/from_into.rs"]
mod from_into;

#[path = "flow_control/index.rs"]
mod flow_control;

fn main() {
    // vectors::vectors();

    // variable::index();

    casting::index();
    mutability::index();
    scoping::index();
    inference::index();
    aliasing::index();
    flow_control::index();

    println!("Hello, world!");
}
