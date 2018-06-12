use std::env;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

#[derive(Debug)]
struct WordStore (HashMap<String, u64>);

impl WordStore {
    fn new()  {
        WordStore ( HashMap::new())
    }

    fn increment(word: &str) {
        let key = word.to_string();
        let count = self.0.entry(key).or_insert(0);
        *count += 1;
    }
}

fn main() {
    let arguments: vec<String> = env::args().collect();
    println!("args 1 {}", arguments[1]);

    let filename = arguments[1].clone();

    let file = File::open(filename).expect("could no open file");
    let reader = BufReader::new(file);

    let word_store = WordStore::new();

    for line in reader.lines() {
        let line = line.expect("Could no read line");
        let words = line.split(" ");
        for word in words {
            if word == "" {
                continue
            } else{
                word_store.increment(word);
            }
        }
    }

    word_store.display();
}