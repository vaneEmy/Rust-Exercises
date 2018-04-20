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

    let mut e: Vec<i32> = Vec::with_capacity(10);
    println!("Length: {}, Capacity: {}", e.len(), e.capacity());

    for i in 0..10{
        e.push(i);
    }

    e.push(11);
    println!("Vector e: {:?}", e);

    let mut v = vec![1, 2, 3, 4, 5];

    for i in &v{
        println!("A reference to {}", i);
    }

    for i in &mut v {
        println!("A mutable reference to {}", i);
    }

    for i in v{
        println!("Take ownership of the vector and its elements {}", i);
    }
}