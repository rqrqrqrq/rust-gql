extern crate juniper;

pub struct Context {
    pub url: String,
}

impl juniper::Context for Context {}
