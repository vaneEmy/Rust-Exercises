pub struct Post{
    state: Option<Box<State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post{
        Post{
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // The add_text method takes a mutable reference to self, 
    // because we're changing the Post instance that we're calling add_text on.
    pub fn add_text(&mut self, text: &str){
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        ""
    }
}

trait State {}

struct Draft {}

impl State for Draft {}





#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
