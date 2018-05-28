fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8,_> = "34".parse();

    if let Some(color) = favorite_color{
         println!("Using your favorite color, {}, as the background", color);
    }
    else if is_tuesday{
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30{
            println!("uUsing purple as he background color");
        }else{
             println!("uUsing orange as he background color");
        }
    }else{
        println!("Using blue as the background color.");
    }
}
