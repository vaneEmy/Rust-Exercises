extern "C"  {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from c");
}

fn main() {
    unsafe{
        println!("Absolute value f -3 according to C: {}", abs(-3));
    }
}
