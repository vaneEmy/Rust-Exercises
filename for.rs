fn main(){
    for a in 0..10{
        println!("Current value: {}", a);
    }

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
}