pub trait Draw {
    fn draw(&self);
}

struct Screen {
    pub components: Vec<Box<Draw>>,
}

impl Screen {
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw();
        }
    }
}

pub struct Button{
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button{
    fn draw(&self){

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
