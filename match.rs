fn main() {
    let tshirt_width = 24;
    let tshirt_size = match tshirt_width{
        16 => "s",
        17 | 18 => "M",
        19 ... 21 => "L",
        22 => "XL",
        _ => "Not available",
    };
    println!("{}", tshirt_size);

    let is_allowed = false;
    let list_type = match is_allowed{
        true => "Full",
        false => "Restricted"
    };
    println!("{}", list_type);
}