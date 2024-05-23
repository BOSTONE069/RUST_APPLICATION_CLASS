pub fn structs_datatypes(){
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User {
        active: true,
        username: String::from("bostone1234"),
        email: String::from("bostone@gmail.com"),
        sign_in_count: 1,
    };

    println!("User details:");
    println!("Active: {}", user1.active);
    println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);
    println!("Sign-in count: {}", user1.sign_in_count);

    fn build_user(email: String, username: String) -> User{
        User {
            active:true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    let user2 = User {
        email: String::from("joy@gmail.com"),
        ..user1
    };
}