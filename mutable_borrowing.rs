fn main(){
    let mut a = [1, 2, 3];
    let b = &mut a;
    b[0] = 4;
    println!("{:?}", b);

}