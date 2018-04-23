struct Player<'a> {
    id: u8,
    name: &'a str
}

impl <'a> Player<'a> {
    fn new(id: u8, name: &str) -> Player{
        Player{
            id: id,
            name: name
        }
    }

    fn heading_text(&self) -> String{
        format!("{}: {}", self.id, self.name)
        
    }

}

fn main(){
    let player1 = Player::new(1, "Serena Williams");
    let player1_heading_text = player1.heading_text();
    println!("{}", player1_heading_text);
}