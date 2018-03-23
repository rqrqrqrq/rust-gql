use gql_context::Context;

#[derive(GraphQLObject)]
struct User {
    id: i32,
    name: String,
}

pub struct Query;

graphql_object!(Query: Context |&self| {
    field echo(x: String) -> String {
        x
    }

    field url(&executor) -> String {
        executor.context().url.clone()
    }

    field User() -> User {
        User {
            id: 1,
            name: "yoba".to_owned(),
        }
    }
});
