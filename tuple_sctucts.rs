struct Color(u8, u8, u8);
struct Kilometers(i32);

fn main(){
    // Creating an instance
    let black = Color(0, 0, 0);

    // destructure the instanace using a 'let' bindind, this will not desctuct black instance
    let Color(r, g, b) = black;
    println!("Black = rgb({}, {}, {})", r, g, b);

    //new type pattern -> When a tuple has only one element
    let distance = Kilometers(20);
    // destructure the instance using a 'let' binding
    let Kilometers(distance_in_km) = distance;
    println!("The distance: {} km", distance_in_km);

}