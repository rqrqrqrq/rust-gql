#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate dotenv;
#[macro_use]
extern crate juniper;
extern crate juniper_rocket;

use dotenv::dotenv;
use std::env;
use rocket::response::content::Html;
use rocket::State;
use juniper::{RootNode, EmptyMutation};
use juniper_rocket::{GraphQLResponse, GraphQLRequest, graphiql_source};

struct Context {
    url: String,
}

impl juniper::Context for Context {}

struct Query;

graphql_object!(Query: Context |&self| {
    field foo() -> String {
        "Hello World".to_owned()
    }

    field url(&executor) -> String {
        executor.context().url.clone()
    }
});

type Schema = RootNode<'static, Query, EmptyMutation<Context>>;

#[post("/graphql", data = "<request>")]
fn graphql(
    request: GraphQLRequest,
    schema: State<Schema>,
    context: State<Context>,
) -> GraphQLResponse {
    request.execute(&schema, &context)
}

#[get("/playground")]
fn playground() -> Html<String> {
    graphiql_source("/graphql")
}

fn main() {
    dotenv().expect("`.env` file is not present");

    let database_url = env::var("DATABASE_URL").expect("`DATABASE_URL` should be in env vars");

    let context = Context { url: database_url };

    rocket::ignite()
        .manage(context)
        .manage(Schema::new(Query, EmptyMutation::<Context>::new()))
        .mount("/", routes![playground, graphql])
        .launch();
}
