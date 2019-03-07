fn main() {

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.email);
    user1.email = String::from("adifferent@adres.se");
    println!("{}", user1.email);

    let email = String::from("adifferent@lala.land");
    let username = String::from("Name");

    let user2 = build_user(email, username);
    println!("{}", user2.email);

    let user3 = User {
        email: String::from("athirdt@adres.se"),
        username: String::from("Name"),
        ..user1

    };
    println!("{}", user3.sign_in_count);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
