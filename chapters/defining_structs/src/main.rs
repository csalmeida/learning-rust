fn main() {
    // This is a template of a user:
    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        username: String::from("usernew"),
        email: String::from("another@example.com"),
        ..user1
    };

    println!("User 1: {:#?}", user1);
}
