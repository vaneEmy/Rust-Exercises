fn main(){
    let a = vec![1, 2, 3];
    let b = get_first_element(&a);

    println!("{:?} {}", a, b);
}

fn get_first_element(a: &Vec<i32>) -> i32{
    a[0]
}