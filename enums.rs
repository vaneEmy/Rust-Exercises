enum FlashMessage{
    Success, //a unit variant
    Warning{category: i32, message: String}, //a struct variant
    Error(String) // a tuple variant
}

fn main(){
    let mut form_status = FlashMessage::Success;
    print_flash_message(form_status);

    form_status = FlashMessage::Warning{category: 2, message: String::from("Field X is required")};
    print_flash_message(form_status);

    form_status = FlashMessage::Error(String::from("Connection Error"));
    print_flash_message(form_status);
}

fn print_flash_message(m: FlashMessage){
    //pattern matching with enum
    match  m {
        FlashMessage::Success => println!("Form submitted correctly"),
        FlashMessage::Warning{category, message} => println!("Warning: {} - {}", category, message),
        FlashMessage::Error(msg) => println!("Error: {}", msg)
    }
}