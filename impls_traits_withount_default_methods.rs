struct Player{
    first_name: String,
    last_name: String,
}
trait FullName{
    fn full_name(&self) -> String;
}

impl FullName for Player{
    fn full_name(&self) -> String{
        format!("{} {} ", self.first_name, self.last_name)
    }
}

fn main(){
    let player_2 = Player{
        first_name: "Roger".to_string(),
        last_name: "Federer".to_string(),
    };
    println!("Player 02: {}", player_2.full_name());
}