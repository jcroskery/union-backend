use actix_web::{get, App, HttpResponse, HttpServer, Responder};

mod static_interface;
mod html_builder;

const PORT: i32 = 8080;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(static_interface::get_static("index.html").await)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Server on port {}", PORT);
    HttpServer::new(|| {
        App::new().service(hello)
    })
    .bind(format!("0.0.0.0:{}", PORT))?
    .run()
    .await
}

