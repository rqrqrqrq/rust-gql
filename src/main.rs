#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

#[get("/")]
fn index() -> String {
    env::var("DATABASE_URL").expect("`DATABASE_URL` should be in env vars")
}

fn main() {
    dotenv().expect("`.env` file is not present");

    rocket::ignite().mount("/", routes![index]).launch();
}
