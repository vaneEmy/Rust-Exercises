fn main(){
    for a in 0..10{
        println!("Current value: {}", a);
    }

    println!("------ Break and continue ------");

    for b in 0..6{
        if b == 0{
            println!("Skip value: {}", b);
            continue;
        }else if b == 2{
            println!("Break At: {}", b);
            break;
        }
        println!("Current value: {}", b);
    }

    println!("------ Outer break-------");

    'outer_for: for c1 in 1..6{
        'inner_for: for c2 in 1..6{
            println!("Current value: [{}] [{}]", c1, c2);
            if c1 == 2 && c2 == 2 { break 'outer_for}
        }
    }
}