pub mod client;

pub mod network;

#[cfg(test)]
mod test{
    #[test] 
    fn it_workd(){
        assert_eq!(2 + 2, 4);
    }
}