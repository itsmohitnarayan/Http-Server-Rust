use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
