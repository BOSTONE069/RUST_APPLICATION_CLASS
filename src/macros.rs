/// This code defines a macro named `hello_world` using the `macro_rules!` macro in Rust. The macro
/// takes no input arguments `()` and when invoked, it will expand to the code `println!("Hello,
/// world!");`. This means that whenever `hello_world!()` is called in the code, it will print "Hello,
/// world!" to the standard output.
macro_rules! hello_world {
    () => {
        println!("Hello, world!")
    };
}


pub fn macros(){
    hello_world!()
}