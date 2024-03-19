use actix_web::{get, patch, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas available!")
}

#[post("/pizzas")]
async fn buy_pizza() -> impl Responder {
    HttpResponse::Created().body("Buying a pizza!")
}

#[patch("/pizzas/{uuid}")]
async fn update_pizza(uuid: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Updating pizza with uuid: {}", uuid))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_pizzas)
            .service(buy_pizza)
            .service(update_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
