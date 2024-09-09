use std::sync::Mutex;
use std::ops::AddAssign;
use std::thread::{spawn, scope};
pub fn test_mutex () {
    let mut score = Mutex::new(0u16);

    // let unlocked_data = score.lock();

    // let mut data = unlocked_data.unwrap();

    // data.add_assign(5);

    // println!("{:?}", data);

    let myfunc = || {
        let mut data = score.lock().unwrap();
        for i in 1..10 {
            data.add_assign(i);
        }
    };


    let myfunc2 = || {
        let mut data = score.lock().unwrap();
        for i in 1..10 {
            data.add_assign(i);
        }
    };


    _ = scope(|s| {
        s.spawn(myfunc);
        s.spawn(myfunc2);

    });

    println!("{:?}", score.lock().unwrap());
}