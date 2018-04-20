fn  main() {
    let mut c = vec![5, 4, 3, 2, 1];
    c[0] = 1;
    c[1] = 2;
    println!("Vector a: {:?}", c);

    let mut d: Vec<i32> = Vec::new();
    d.push(1);
    d.push(2);
    d.pop();

    println!("Vector d: {:?}", d);

}