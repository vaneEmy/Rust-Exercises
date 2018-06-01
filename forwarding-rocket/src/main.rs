#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/hello/<name>/<age>/<cool>")]
fn hello(name: String, age: u8, cool: bool) -> String {
    format!("Hello {}, your age is: {}", name, age)    
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}
