use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config{
    pub query: String,
    pub filename: String,
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}


pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
   
    f.read_to_string(&mut contents)?;
    
    Ok(())
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\n
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(
            vec!["sage, fast, productive."], 
            search(query, contents)
            );
    }
}