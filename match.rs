fn main() {
    let tshirt_width = 24;
    let tshirt_size = match tshirt_width{
        16 => "s",
        17 | 18 => "M",
        19 ... 21 => "L",
        22 => "XL",
        _ => "Not available",
    };
    println!("Shirt size: {}", tshirt_size);

    let is_allowed = false;
    let list_type = match is_allowed{
        true => "Full",
        false => "Restricted"
    };
    println!("List type: {}", list_type);

    let marks_paper_a: u8 = 25;
    let marks_paper_b: u8 = 30;
    let output = match(marks_paper_a, marks_paper_b){
        (50, 50) => "Full marks for both papers",
        (50, _) => "Full marks for paper A",
        (_, 59) => "Full marks for paper B",
        (x, y) if x > 25 && y > 25 => "Good",
        (_,_) => "Work hard"
    };

    println!("{}", output);
}