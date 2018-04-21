struct Player{
    first_name: String,
    last_name: String,
}

impl Player{
    fn full_name(&self) -> String {
     format!("{} {}", self.first_name, self.last_name)   
    }
}

fn main(){
    let player_1 = Player{
        first_name: "Rafael".to_string(),
        last_name: "Nadal".to_string(),
    };

    println!("Player 01: {}", player_1.full_name());
}