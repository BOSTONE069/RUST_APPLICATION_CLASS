use std::fmt;


enum IpAddrKind{
    V4(String),
    V6(String),
}

/// This `enum Message` is defining a Rust enum called `Message` that represents different types of
/// messages. It has four variants:
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

/// This `impl Message` block is implementing a method called `process` for the `Message` enum. The
/// `process` method takes a reference to `self` (an instance of `Message`) and then matches on the
/// different variants of the `Message` enum.
impl Message {
    fn process(&self) {
        match self {
            Message::Quit => {
                println!("Quitting the program");
            },
            Message::Move {x, y} => {
                println!("Moving to ({}, {})", x, y);
            },
            Message::Write(text) => {
                println!("Writing: {}", text);
            },
            Message::ChangeColor(r, g, b) => {
                println!("Changing color to RGB({}, {}, {})", r, g, b);
            },
        }
    }
}

pub fn enumms(){

    let quit_message = Message::Quit;
    let move_message = Message::Move {x: 10, y: 20};
    let write_message = Message::Write(String::from("Hello, Rust!"));
    let color_message = Message::ChangeColor(255, 0, 0);

    quit_message.process();
    move_message.process();
    write_message.process();
    color_message.process();


    const AGE: i32 = 13;

    enum STATUS {
        ADULT,
        CHILD,
    }

    // Implement the Display trait for STATUS
    impl fmt::Display for STATUS {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                STATUS::ADULT => write!(f, "ADULT"),
                STATUS::CHILD => write!(f, "CHILD"),
            }
        }
    }
    let user_status = if AGE > 18 {
        STATUS::ADULT
    } else {
        STATUS::CHILD
    };

    println!("The status of this person is {}", user_status);

    let _home = IpAddrKind::V4(String::from("127.0.0.1"));

    let _loopback = IpAddrKind::V6(String::from("::1"));
}