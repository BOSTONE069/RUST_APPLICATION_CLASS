pub fn test_match_int(){
    let myage: u16 = 35;

    match myage {
        1..= 35 => { //Range pattern matching
            println!("Your age is 35");
        }
        _ => {
            println!("You age is not 35 ");
        }
    }
}