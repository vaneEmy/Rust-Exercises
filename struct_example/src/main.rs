struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let email1 = String::from("anotheremail@example.com");
    let username1 = String::from("someoneusername123");
    let user1 = build_user(email1, username1);

    let user2 = User{
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
}

fn build_user(email: String, username: String) -> User{
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,   
    }
}