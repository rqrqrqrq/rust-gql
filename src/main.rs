extern crate dotenv;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().expect("`.env` file is not present");

    let database_url = env::var("DATABASE_URL").expect("`DATABASE_URL` should be in env vars");

    println!("Hello, world! {:?}", database_url);
}
