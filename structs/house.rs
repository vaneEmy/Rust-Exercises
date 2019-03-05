struct Room{
    is_upstairs: bool,
    room_area: Area,
    window: Vec<Window>,
    number_of_door: i32,
    wood_or_carpet: bool,
    carpet_color: String,
    room_name: String,
    has_wardrobe: bool,
}

struct House{
    rooms: Vec<Room>
}

struct Area{
    width: f32,
    length: f32,
}

struct Window{
    window_area: Area,
    window_type: String,
    has_blinds: bool,
    curtains_color: String,
    has_lock: bool,
    top_open: bool,
    single_window: bool,
}

fn main(){
    let mut room = Room {
        is_upstairs: true,
        room_area: Area { width: 2.3f32, length: 4.3f32},   
        number_of_door: 1,
        wood_or_carpet: true,
        carpet_color: "Red".to_string(),
        room_name: "bedroom 1".to_string(),
        has_wardrobe: true,
        window: vec![
            Window{
                window_area: Area {width: 1.3f32, length: 1.4f32},
                window_type: "Main".to_owned(),
                has_blinds: true,
                curtains_color: "Blue".to_owned(),
                has_lock: false,
                top_open: true,
                single_window: true,
            },
            Window{
                 window_area: Area {width: 0.9f32, length: 1.1f32},
                window_type: "Small".to_owned(),
                has_blinds: true,
                curtains_color: "Blue".to_owned(),
                has_lock: false,
                top_open: true,
                single_window: true,
            }
        ]
    };

    println!("Bedroom {} has {} door", room.room_name, room.number_of_door);
    println!("The room width is {} m  by {}m", room.room_area.width, room.room_area.length);
    let ref window_two = room.window[1];
    println!("Window 2 is {}m by {}m has {} curtains", window_two.window_area.width, window_two.window_area.length, window_two.curtains_color);

}
