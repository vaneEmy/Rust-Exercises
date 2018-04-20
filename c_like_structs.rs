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

    let Color {red: r, green: g, blue:b} = blue;
    print!("Blue = rgb({}, {}, {})", r, g, b);
}