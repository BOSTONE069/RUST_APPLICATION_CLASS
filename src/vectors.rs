/// The `vectors` function in Rust creates a vector, accesses a specific element by index, and prints
/// the value if found.
pub fn vectors(){
    let my_vec = vec![1,2,3,4,5];

    let result_2 = my_vec.get(3);
     match result_2 {
        Some(number) => {
            println!("The value at index 3 is: {}", number);
        },
        None => {
            println!("No value found at index 3");
        }
    }
}