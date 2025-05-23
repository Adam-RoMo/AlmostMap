use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(root))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
