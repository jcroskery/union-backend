use actix::prelude::*;
use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use mysql::params;
use mysql::prelude::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rand_core::OsRng;
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use scrypt::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt, Params, 
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::fs::File;
use std::io::BufReader;
use union_structs::{Login, Signup};

mod html_builder;
mod mysql_init;
mod static_interface;
mod union_structs;

const HTTPPORT: i32 = 80;
const HTTPSPORT: i32 = 443;
const PUBCERT: &str = "/etc/letsencrypt/live/union.tk/fullchain.pem";
const KEY: &str = "/etc/letsencrypt/live/union.tk/privkey.pem";

#[derive(Deserialize)]
struct Info {
    name: String,
}
struct MyWs {
    url: String,
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

fn handle_signup(json: Value) -> Value {
    let signup: Signup = serde_json::from_value(json).expect("Invalid Signup JSON");
    if let (Some(email), Some(password), Some(username)) = (
        signup.get_email(),
        signup.get_password(),
        signup.get_username(),
    ) {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Scrypt
            .hash_password(password.as_bytes(), None, Params::new(12, 8, 1).unwrap(), &salt)
            .unwrap()
            .to_string();
        mysql_init::get_conn()
            .exec_drop(
                "INSERT INTO users(email, password, username) VALUES (:email, :password, :username);",
                params!("email"=>email, "password"=>password_hash, "username"=>username),
            )
            .expect("Failed to execute signup mysql statement");
        json!({ "success": true })
    } else {
        json!({"success" : false})
    }
}

fn handle_login(json: Value) -> Value {
    let login: Login = serde_json::from_value(json).expect("Invalid Login JSON");
    if let (Some(email), Some(password)) = (login.get_email(), login.get_password()) {
        let selected_user_row: Vec<mysql::Row> = mysql_init::get_conn()
            .exec(
                "SELECT * FROM users WHERE email=:email;",
                params!("email"=>email),
            )
            .expect("Failed to execute login mysql statement");
        if selected_user_row.len() == 1 {
            let password_hash: String = mysql::from_value(selected_user_row[0]["password"].clone());
            let user_id: i32 = mysql::from_value(selected_user_row[0]["id"].clone());
            let parsed_hash = PasswordHash::new(&password_hash).unwrap();
            if let Ok(_) = Scrypt.verify_password(password.as_bytes(), &parsed_hash) {
                let vec: Vec<u8> = thread_rng().sample_iter(&Alphanumeric).take(255).collect();
                let id = String::from_utf8(vec).expect("RNG error");
                mysql_init::get_conn().exec_drop("DELETE FROM activesessions WHERE user=:userid", params!("userid" => &user_id)).expect("Failed to delete old ids");
                mysql_init::get_conn().exec_drop("INSERT INTO activesessions VALUES (:id, :userid);", params!("id"=>&id, "userid"=>user_id)).expect("Failed to init id");
                return json!({"success": true, "id": id});
            }
        }
    }
    json!({"success": false})
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                let json: serde_json::Value = serde_json::from_str(&text).expect("No JSON format");

                let returned_json = match self.url.as_str() {
                    "login" => handle_login(json),
                    "signup" => handle_signup(json),
                    _ => {
                        serde_json::json!({
                            "success": false,
                            "message": "URL does not exist"
                        })
                    }
                };
                ctx.text(serde_json::to_string(&returned_json).expect("Failed to Stringify JSON"))
            }
            _ => (),
        }
    }
}

async fn ws_response(
    info: web::Path<Info>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    ws::start(
        MyWs {
            url: info.name.clone(),
        },
        &req,
        stream,
    )
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
    mysql_init::create_tables().expect("Failed to initialize tables");
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
