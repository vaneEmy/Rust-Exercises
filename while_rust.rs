fn main(){
    let mut a = 1;
    while a <= 10{
        println!("Current value: {}", a);
        a += 1;
    }

    let mut b = 0;
    while b < 5{    
        if b == 0{
            println!("Skip value: {}", b);
            b += 1;
            continue;
        } else if b == 2{
            println!("Break at: {}", b);
            break;
        }
        println!("Current value: {}", b);
        b += 1;
    }

}