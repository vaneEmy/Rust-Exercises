use std::rc::Rc;
use std::cell::RefCell;
use List::{Cons, Nil};

#[derive(Debug)]
enum List{
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&refCell<Rc<List>>>{
        match *self{
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }

}

fn main() {
    println!("Hello, world!");
}
