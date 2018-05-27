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

    pub fn request_review(&mut self){
        if let Some(s) = self.state.take(){
            self.state = Some(s.request_review());
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<State>{
        Box::new(PendingReview{})
    }
}

struct PendingReview {}

impl State for PendingReview{
    fn request_review(self: Box<Self>) -> Box<State>{
        self
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
