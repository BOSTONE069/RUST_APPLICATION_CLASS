use core::fmt;

use fmt::Debug;

#[derive(Debug)]
/// The struct `User` in Rust represents a user with fields for activity status, username, email, and
/// sign-in count.
/// 
/// Properties:
/// 
/// * `active`: The `active` property in the `User` struct is a boolean value that indicates whether the
/// user account is currently active or not. If `active` is `true`, it means the user account is active
/// and can be used for sign-ins and other operations. If `active` is `false
/// * `username`: The `username` property in the `User` struct represents the username of a user. It is
/// of type `String` and stores the user's chosen username for identification purposes.
/// * `email`: The `email` property in the `User` struct represents the email address associated with a
/// user account. It is of type `String` and is used for communication and identification purposes.
/// * `sign_in_count`: The `sign_in_count` property in the `User` struct represents the number of times
/// the user has signed in to their account. It is of type `u64`, which is an unsigned 64-bit integer in
/// Rust. This property can be used to track the login activity of the user.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn structs_datatypes(){


    // The code `let mut user1 = User { active: true, username: String::from("bostone1234"), email:
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

    /// The function `build_user` creates a new `User` instance with the provided email, username, and
    /// default sign-in count.
    /// 
    /// Arguments:
    /// 
    /// * `email`: Email address of the user.
    /// * `username`: The `username` parameter in the `build_user` function is a `String` type
    /// representing the username of the user being created.
    /// 
    /// Returns:
    /// 
    /// A `User` struct is being returned with the provided `email` and `username` values, along with
    /// default values for `active` set to `true` and `sign_in_count` set to `1`.
    fn build_user(email: String, username: String) -> User{
        User {
            active:true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    // This code snippet is using struct update syntax in Rust.
    let user2 = User {
        email: String::from("joy@gmail.com"),
        ..user1
    };

    println!("{:?}", user2);
}