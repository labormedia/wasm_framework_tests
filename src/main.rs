fn main() {
    println!("Hello, world!");
}

#[no_mangle]
pub extern "C" fn print_hello() {
    println!("Hello, world! no mangle!");
}