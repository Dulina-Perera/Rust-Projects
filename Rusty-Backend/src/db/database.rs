use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub db_name: String,
    pub name_space: String,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;

        client.signin(Root {
            password: "root",
            username: "root",
        }).await?;

        client.use_ns("surreal").use_db("pizzas").await.unwrap();

        Ok(Database {
            client,
            db_name: String::from("pizzas"),
            name_space: String::from("surreal"),
        })
    }
}