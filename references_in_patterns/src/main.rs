fn main() {
    let robot_name = Some(String::from("Bors"));

    match robot_name {
        Some(name) => println!("Found a name: {}", name),
        None => (),
    }
    println!("Robot_name is {:?}", robot_name);
}
