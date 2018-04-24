fn main(){
    greetings::hello();
}

mod greetings{
    pub fn hello(){
        println!("Hello world!");
    }
}