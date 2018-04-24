fn main(){
    phrases::greet();
}

mod phrases{
    pub fn greet(){
        hello();
    }

    fn hello(){
        println!("Hello, world!");
    }
   
}