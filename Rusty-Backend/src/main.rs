mod db;
mod models;

use actix_web::{get, patch, post, web, web::Data, web::Json, App, HttpResponse, HttpServer, Responder};
use crate::db::Database;
use crate::models::PizzaRequest;
use validator::Validate;

#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas available!")
}

#[post("/pizzas")]
async fn buy_pizza(body: Json<PizzaRequest>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name = &body.name;
            HttpResponse::Created().body(format!("Buying a {}!", pizza_name))
        },
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[patch("/pizzas/{uuid}")]
async fn update_pizza(uuid: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Updating pizza with uuid: {}", uuid))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init().await.expect("Failed to initialize database!");
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_pizzas)
            .service(buy_pizza)
            .service(update_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
