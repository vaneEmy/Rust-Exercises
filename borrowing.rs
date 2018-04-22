fn main(){
    let mut a = vec![1, 2, 3];
    {
        let b = &mut a;
        println!("Value b: {:?}", b);
    }
    println!("Value a: {:?}", a);
}