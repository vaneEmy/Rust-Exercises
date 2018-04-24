fn main(){
    phrases::greeting::hello();    
}

mod phrases{
    fn private_fn(){
        println!("Hello, world!");
    }

    pub mod greeting{
        pub fn hello(){
            super::private_fn();
        }
    }
}