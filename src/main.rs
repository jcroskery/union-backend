use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

mod html_builder;
mod static_interface;

const PORT: i32 = 80;

#[derive(Deserialize)]
struct Info {
    name: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(static_interface::get_static("index.html").await.expect("Couldn't find index.html"))
}

async fn static_response(info: web::Path<Info>) -> impl Responder {
    println!("Got request for {}", &info.name);
    HttpResponse::Ok().body(static_interface::get_static(&info.name).await.unwrap_or(String::new()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Server on port {}", PORT);
    HttpServer::new(|| {
        App::new().service(hello).service(
            web::resource("/{name:.*}")
                .route(web::get().to(static_response))
        )
    })
    .bind(format!("0.0.0.0:{}", PORT))?
    .run()
    .await
}
