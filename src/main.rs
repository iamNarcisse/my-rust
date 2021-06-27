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

#[path = "custom_types/index.rs"]
mod custom_types;

#[path = "functions/index.rs"]
mod functions;

#[path = "error_handling/index.rs"]
mod error_handling;

#[path = "error_handling/combinators.rs"]
mod combinators;

fn main() {
    // vectors::vectors();

    // variable::index();

    // casting::index();
    // mutability::index();
    // scoping::index();
    // inference::index();
    // aliasing::index();
    // flow_control::index();
    // custom_types::index();
    // functions::main();
    error_handling::main();
    combinators::combinators();

    println!("Hello, world!");
}
