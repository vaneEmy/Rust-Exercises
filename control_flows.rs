fn main(){
  let team_size = 7;
  let team_size_in_text;
  if team_size < 5{
     team_size_in_text = "Small";
  } else if team_size < 10{
    team_size_in_text = "Medium";
  }else{
    team_size_in_text = "Large";
  }
  println!("Current team size: {}", team_size_in_text);
}
