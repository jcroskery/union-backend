use actix_web::{get, App, HttpResponse, HttpServer, Responder};

const PORT: i32 = 8080;
const MESSAGE: &str = "Hello world!";

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(MESSAGE)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Server");
    HttpServer::new(|| {
        App::new().service(hello)
    })
    .bind(format!("0.0.0.0:{}", PORT))?
    .run()
    .await
}

