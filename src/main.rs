use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

mod html_builder;
mod static_interface;

const HTTPPORT: i32 = 80;
const HTTPSPORT: i32 = 443;
const PUBCERT: &str = "/etc/letsencrypt/live/union.tk/fullchain.pem";
const KEY: &str = "/etc/letsencrypt/live/union.tk/privkey.pem";

#[derive(Deserialize)]
struct Info {
    name: String,
}

async fn static_response(info: web::Path<Info>) -> impl Responder {
    let name = if info.name.chars().rev().next().unwrap_or('/') == '/' {
        format!("{}index.html", &info.name)
    } else {
        info.name.clone()
    };
    println!("Got request for {}", name);
    HttpResponse::Ok().body(
        static_interface::get_static(&name)
            .await
            .unwrap_or(String::new()),
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open(PUBCERT).unwrap());
    let key_file = &mut BufReader::new(File::open(KEY).unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = pkcs8_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    println!("Starting Server on ports {} and {}", HTTPPORT, HTTPSPORT);
    
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/{name:.*}").route(web::get().to(static_response)))
    })
    .bind_rustls(format!("0.0.0.0:{}", HTTPSPORT), config)?
    .bind(format!("0.0.0.0:{}", HTTPPORT))?
    .run()
    .await
}
