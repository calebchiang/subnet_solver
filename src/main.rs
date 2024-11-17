use actix_web::{web, App, HttpServer};
mod handlers;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(handlers::welcome))
        .route("/subnet/{ip}/{subnet_mask}", web::get().to(handlers::calculate_subnet))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}