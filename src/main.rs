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
    Params, Scrypt,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::fs::File;
use std::io::BufReader;
use union_structs::{GalleryCreate, Login, Signup};

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

#[derive(Deserialize)]
struct GalleryInfo {
    username: String,
    gallery: String,
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
            .hash_password(
                password.as_bytes(),
                None,
                Params::new(12, 8, 1).unwrap(),
                &salt,
            )
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
                mysql_init::get_conn()
                    .exec_drop(
                        "DELETE FROM activesessions WHERE user=:userid",
                        params!("userid" => &user_id),
                    )
                    .expect("Failed to delete old ids");
                mysql_init::get_conn()
                    .exec_drop(
                        "INSERT INTO activesessions VALUES (:id, :userid);",
                        params!("id"=>&id, "userid"=>user_id),
                    )
                    .expect("Failed to init id");
                return json!({"success": true, "id": id});
            }
        }
    }
    json!({"success": false})
}

fn handle_gallery_creation(json: Value) -> Value {
    let gallery_create: GalleryCreate =
        serde_json::from_value(json).expect("Invalid Gallery Creation JSON");
    if let (Some(gallery_name), Some(id)) =
        (gallery_create.get_gallery_name(), gallery_create.get_id())
    {
        if let Some(user_row) = authenticate_with_id(id) {
            let userid: i32 = mysql::from_value(user_row["id"].clone());
            mysql_init::get_conn().exec_drop("INSERT INTO galleries(user, name) VALUES (:user, :name);", params!("user"=> userid, "name"=>gallery_name)).expect("Failed to create gallery");
            return json!({"success": true});
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
                    "creategallery" => handle_gallery_creation(json),
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

fn authenticate_with_id(id: String) -> Option<mysql::Row> {
    let active_sessions: Vec<mysql::Row> = mysql_init::get_conn()
        .exec(
            "SELECT * FROM activesessions WHERE id=:userid;",
            params!("userid"=>id),
        )
        .expect("Failed to get activesessions");
    if active_sessions.len() == 1 {
        let userid: i32 = mysql::from_value(active_sessions[0]["user"].clone());
        let matching_users: Vec<mysql::Row> = mysql_init::get_conn()
            .exec("SELECT * FROM users WHERE id=:id", params!("id"=>userid))
            .expect("Failed to get active users.");
        if matching_users.len() == 1 {
            return Some(matching_users[0].clone());
        }
    }
    None
}

async fn authenticate(hr: HttpRequest) -> Option<mysql::Row> {
    if let Some(cookie) = hr.cookie("id") {
        if let Some(id) = union_structs::parse(&union_structs::ID_REGEX, cookie.value()) {
            return authenticate_with_id(id);
        }
    }
    None
}

async fn userpage_response(info: web::Path<Info>, hr: HttpRequest) -> impl Responder {
    if let Some(user_row) = authenticate(hr).await {
        let username: String = mysql::from_value(user_row["username"].clone());
        let userid: i32 = mysql::from_value(user_row["id"].clone());
        if username == info.name {
            let user_galleries: Vec<mysql::Row> = mysql_init::get_conn()
                .exec(
                    "SELECT * FROM galleries WHERE user=:userid;",
                    params!("userid"=>userid),
                )
                .expect("Failed to get user galleries");
            let gallery_names: Vec<String> = user_galleries
                .into_iter()
                .map(|user_gallery| mysql::from_value(user_gallery["name"].clone()))
                .collect();
            return HttpResponse::Ok()
                .body(static_interface::get_user_page(&username, gallery_names).await);
        }
    }
    HttpResponse::Ok().body("")
}

async fn gallery_response(info: web::Path<Info>, hr: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("")
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
            .service(
                web::resource("/u/{username}/{gallery}").route(web::get().to(gallery_response)),
            )
            .service(web::resource("/u/{name}").route(web::get().to(userpage_response)))
            .service(web::resource("/ws/{name}").route(web::get().to(ws_response)))
            .service(web::resource("/{name:.*}").route(web::get().to(static_response)))
    })
    .bind_rustls(format!("0.0.0.0:{}", HTTPSPORT), config)?
    .bind(format!("0.0.0.0:{}", HTTPPORT))?
    .run()
    .await
}
