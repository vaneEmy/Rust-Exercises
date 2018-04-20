struct Color{
    red: u8,
    green: u8,
    blue: u8
}

fn main(){
    // creating an instance
    let black = Color{red: 0, green: 0, blue: 0};

    // accessing its fields using dot notation
    println!("Black = rgb({}, {}, {})", black.red, black.green, black.blue);

    // structs are immutable by default.
    let mut link_color = Color{red: 0, green: 0, blue: 255};
    link_color.blue = 238;
    println!("Link color = rgb({}, {}, {})", link_color.red, link_color.green, link_color.blue);

    // copy elements from another instance
    let blue = Color{ blue:230, .. link_color};
    println!("Blue = rgb({}, {}, {})", blue.red, blue.green, blue.blue);

    // destructure the instance using a 'let' binding, this will not desctruct blue instance
    let Color {red: r, green: g, blue:b} = blue;
    println!("Blue = rgb({}, {}, {})", r, g, b);

    // creating an instance via functions & accessing its fields
    let midnightblue = get_midnightblue_color();
    println!("Midnight Blue = rgb({}, {}, {})", midnightblue.red, midnightblue.green, midnightblue.blue); //Midnight Blue = rgb(25, 25, 112)}

}

fn get_midnightblue_color() -> Color{
    Color {red: 25, green: 25, blue: 122}
}