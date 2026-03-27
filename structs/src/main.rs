fn main() {



    let mut user1 = User { // note is mutable struct
        username: String::from("aknutson"),
        email: String::from("adknutson1175@gmail.com"),
        sign_in_count: 2,
        active: true,
    };

    user1.sign_in_count = 30; // update the field of user1

    println!("{}, {}, {}, {}", user1.email, user1.username, user1.active, user1.sign_in_count);

    let user2 = build_user(String::from("atrainseattle@gmail.com"), String::from("adknutson2233"));

    println!("{}", user2.username);

    let user3 = User {
        email: String::from("aknutson@yahoo.com"),
        ..user2
    };

    println!("{}", user3.username)




}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> crate::User {
    crate::User {
        username,
        email,
        sign_in_count: 0,
        active: true,
    }
}
