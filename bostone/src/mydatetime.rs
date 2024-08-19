use std::{time::{Duration, Instant}, ops::Sub};
use std::thread::sleep;

pub fn test_std_time() {
    let dur1 = Duration::from_secs(15);

    println!("{}", dur1.as_millis());

    let dur2 = Duration::from_millis(14500);

    let dur3 = dur1.checked_sub(dur2);

    println!("{}", dur3.unwrap_or_default().as_millis());

    let now = Instant::now();

    sleep(Duration::from_millis(200));

    println!("Elapsed time is: {}", now.elapsed().as_micros());  
}

pub fn test_chrono(){
    
}