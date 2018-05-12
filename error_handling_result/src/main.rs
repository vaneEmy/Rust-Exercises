use std::io::Read;
use std::fs::File;
use std::io;

fn main() {

}

fn read_username_from_file() -> Result<String, io::Error>{
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;
 
    Ok(s)
}