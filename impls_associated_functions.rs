struct Player{
    first_name: String,
    last_name: String,
}

impl Player{
    fn new(first_name: String, last_name: String) -> Player{
        Player{
            first_name: first_name,
            last_name: last_name,
        }
    }

    fn full_name(&self) -> String{
        format!("{} {} ", self.first_name, self.last_name)
    }
}

fn main(){
    let player_name = Player::new("Serena".to_string(), "Williams".to_string()).full_name();
    println!("Player: {}", player_name);
}