#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rand;
extern crate rocket;

mod paste_id;

use paste_id::PasteID;

#[get("/")]
fn index() -> &'static str {
    "   
    USAGE

    POST /
    
        accepts raw data in the body of the request and responds with a URL of
        a page containing the body's content

    GET /<id>
        
        retrieves the content for te paste with id ´<id>´
    
    "
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
