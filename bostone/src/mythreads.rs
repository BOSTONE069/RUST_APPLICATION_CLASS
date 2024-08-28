use std::thread::spawn;

pub fn test_threads() {
    let mut x = 0u128;
    for i in 1..500 {
        x += i;
    }
}

/// The function `spawn_thread` creates two threads that perform a calculation and checks if both
/// threads have finished before exiting.
pub fn spawn_thread() {
    let thread_fn = || {
        let mut x = 0u128;
        for i in 1..500_000 {
            x += i;
        println!("{}", x);
    }
    };

    let handle = spawn(thread_fn);
    let handle2 = spawn(thread_fn);

    test_threads();

    // handle.join();
    // handle2.join();

    loop {
        test_threads();
        if handle.is_finished() && handle2.is_finished() {
            println!("All the worker are done, lets get out of here!");
        }
        break;
    }

}