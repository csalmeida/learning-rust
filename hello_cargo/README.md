# Cargo
Cargo is the package manager for Rust. It manages project dependencies mostly refered to as crates. Cargo relies on a `toml` file to keep track of which crates the project needs.

Running `cargo run` will download any crates that the project depends on, compile and run the program. Alternatively you might want to use `cargo build` to just compile the project or `cargo test` to check if there are no errors without going through compilation.

Event though `rustc` could be used to run programs Cargo is used most of the time. 