fn main(){
    let mut a = vec![1, 2, 3];
    let b = change_and_get_first_element(&mut a);

    println!("{:?} {}", a, b);
}

fn change_and_get_first_element(a: &mut Vec<i32>) -> i32{
    a[0] = 4;
    a[0]    
}
