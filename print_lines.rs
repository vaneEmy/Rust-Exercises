fn main(){
  println!("{}, {}!", "Hello", "World");
  println!("{0}, {1}!", "Hello", "World");
  println!("{greeting}, {name}!", greeting="Hello", name="world");

  println!("{:?}", [1,2,3]);
  println!("{:#?}", [1,2,3]);

  let x = format!("{}, {}", "Hello", "World");
  println!("{}", x);
}
