use actix_web::{HttpServer, App};
use actix_web::{get, post, web, HttpResponse, Responder};
use data_access::postg;
use http::RequestColor;

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
            .route("/color", web::post().to(insere_color))
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
    HttpResponse::Ok().body( format!("{}",usuario.nome))
}

async fn insere_usuario(rusuario: web::Json<Usuario>) -> impl Responder{
    let mut usuario: postg::Usuario = Usuario {id:0, nome:String::new(), email:String::new(), senha:String::new()};

    usuario.nome=rusuario.nome.to_string();
    usuario.email=rusuario.email.to_string();
    usuario.senha=rusuario.senha.to_string();

    match postg::insert_usuario(&mut usuario).await {
        Ok(_) => {},
        Err(e) =>{ println!("insere_usuario erro: {} ", e)},
    }
    if usuario.id==0 {
        HttpResponse::Ok().body( "{\"erro\":\"email já cadastrado\"}")
    } else {
        HttpResponse::Ok().body( format!("inserido. {}", serde_json::to_string(&usuario).unwrap()))
    }
    
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn insere_color(rcolor: web::Json<RequestColor>) -> impl Responder {
    println!("/color = {}",&rcolor.color);
    match  postg::insert_color(&rcolor.color).await {
        Ok(_) => {},
        Err(e) => println!("Erro na insert_color: {}", e)
    }
    HttpResponse::Ok().body(rcolor.color.to_string())
}