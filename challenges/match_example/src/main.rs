fn main() {
    println!("Match Example");
    show_default_view("admin");
}

// fn show_default_view(role: &str) {
//     if role == "admin" {
//         get_posts();
//     } else if role == "private" {
//         redirect_to("/401");
//     } else {
//         get_dashboard();
//     }
// }

/// Uses pattern matching to run a range of different functions depending on the role passed to it.
/// The definition of the matching function is as follows:
///```rust
/// fn show_default_view(role: &str) {
///    match role {
///        "admin" => get_posts(),
///        "private" => redirect_to("/401"),
///        _ => get_dashboard(),
///    }
/// }
///```
///
/// The same functionality could be implemented with an if statement, albeit more verbose:
///
///```rust
/// fn show_default_view(role: &str) {
///     if role == "admin" {
///         get_posts();
///     } else if role == "private" {
///         redirect_to("/401");
///     } else {
///         get_dashboard();
///     }
/// }
///```
fn show_default_view(role: &str) {
    match role {
        "admin" => get_posts(),
        "private" => redirect_to("/401"),
        _ => get_dashboard(),
    }
}

fn get_posts() {}
fn redirect_to(_: &str) {}
fn get_dashboard() {}
