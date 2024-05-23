use std::fmt;

pub fn enumms(){
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
}