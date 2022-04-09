use actix_web::{HttpServer, App};
use actix_web::{get, post, web, HttpResponse, Responder};
use data_access::postg;
use service_mod::usuario_service::insere_usuario_srv;

use crate::data_access::postg::Usuario;
use crate::service_mod::usuario_service::login_service;

extern crate serde_json;

mod data_access;
mod http;
mod service_mod;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server started on 8080 port..");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)            
            .route("/hey", web::get().to(manual_hello))
            .route("/usuario", web::post().to(insere_usuario))
            .route("/login", web::post().to(login))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn login(rusuario: web::Json<Usuario>) -> impl Responder {
    let usuario= login_service(rusuario.email.to_string(), rusuario.senha.to_string()).await;
    //HttpResponse::Ok().body( format!("{}",usuario.nome))
    HttpResponse::Ok().body( format!("inserido. {}", serde_json::to_string(&usuario).unwrap()))
}

async fn insere_usuario(rusuario: web::Json<Usuario>) -> impl Responder{
    let mut usuario: postg::Usuario = 
        Usuario {id:0, 
            nome:rusuario.nome.to_string(), 
            email:rusuario.email.to_string(), 
            senha:rusuario.senha.to_string()};

    let usuario_saida = insere_usuario_srv(&mut usuario).await;

    if usuario.id==0 {
        HttpResponse::Ok().body( "{\"erro\":\"email já cadastrado\"}")
    } else {
        HttpResponse::Ok().body( format!("{}", serde_json::to_string(&usuario_saida).unwrap()))
    }    
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
