struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let origin = Point { x: 0, y: 0, z: 0};
    let numbers = (2, 4, 6, 16, 32);

    match origin{
        Point {x, .. } => println!("x is {}", x), 
    }

    match numbers{
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }
}