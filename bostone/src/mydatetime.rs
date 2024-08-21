use std::{time::{Duration, Instant}, ops::Sub};
use std::thread::sleep;

extern crate chrono;

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
    let utc_now = chrono::Utc::now();
    println!("{}", utc_now.format("%Y-%b-%d %H:%M:%S"));


    let local_time = chrono::Local::now();
    println!("{}", local_time.format("%Y-%b-%d %H:%M:%S"));

}