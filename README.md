Guessing Game
A simple number guessing game from the Rust Book.

How to Run
cargo run
Changes from the Rust Book
I updated the random number generation:

Old version:
let secret_number = rand::thread_rng().gen_range(1..=100);
New version:
let secret_number: u32 = rand::rng().random_range(1..=100);
Requirements
Rust
rand crate
Use cargo build to build the project.
