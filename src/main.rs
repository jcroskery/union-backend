use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

mod html_builder;
mod static_interface;
mod mysql_init;

const HTTPPORT: i32 = 80;
const HTTPSPORT: i32 = 443;
const PUBCERT: &str = "/etc/letsencrypt/live/union.tk/fullchain.pem";
const KEY: &str = "/etc/letsencrypt/live/union.tk/privkey.pem";

#[derive(Deserialize)]
struct Info {
    name: String,
}
struct MyWs {
    url: String
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                let json: serde_json::Value = serde_json::from_str(&text).expect("No JSON format");

                let response_json = serde_json::json!({"success" : true});
                ctx.text(serde_json::to_string(&response_json).expect("Failed to Stringify JSON"));
            },
            _ => (),
        }
    }
}

async fn ws_response(info: web::Path<Info>, req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(MyWs { url: info.name.clone() }, &req, stream)
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
    mysql_init::create_tables().await.expect("Failed to initialize tables");
    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open(PUBCERT).unwrap());
    let key_file = &mut BufReader::new(File::open(KEY).unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = pkcs8_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    println!("Starting Server on ports {} and {}", HTTPPORT, HTTPSPORT);
    
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/ws/{name}").route(web::get().to(ws_response)))
            .service(web::resource("/{name:.*}").route(web::get().to(static_response)))
    })
    .bind_rustls(format!("0.0.0.0:{}", HTTPSPORT), config)?
    .bind(format!("0.0.0.0:{}", HTTPPORT))?
    .run()
    .await
}
