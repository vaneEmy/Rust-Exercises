pub trait Draw {
    fn draw(&self);
}

struct Screen {
    pub components: Vec<Box<Draw>>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
