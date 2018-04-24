mod phrases{
    pub mod greetings{
        pub fn hello(){
            println!("Hello, world!");
        }
    }
}

fn main(){
    phrases::greetings::hello();
}