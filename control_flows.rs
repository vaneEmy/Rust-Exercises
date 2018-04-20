fn main(){
  let team_size = 7;
  let team_size_in_text = if team_size < 5{
    "Small"
    } else if team_size < 10{
    "Medium"
  }else{
    "Large"
  };
  println!("Current team size : {}", team_size_in_text);
}
