use actix_web::{HttpServer, App};
use actix_web::{get, post, web, HttpResponse, Responder};
use data_access::postg;
use http::RequestColor;

use crate::data_access::postg::Usuario;

mod data_access;
mod http;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server started..");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)            
            .route("/hey", web::get().to(manual_hello))
            .route("/color", web::post().to(insere_color))
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

//async fn login() -> impl Responder {
async fn login(rusuario: web::Json<Usuario>) -> impl Responder {
    //println!("/login {}, {}", rusuario.email, rusuario.senha);

    let mut usuario: postg::Usuario = Usuario {nome:String::new(), email:String::new(), senha:String::new()};
    
    match postg::get_usuario(rusuario.email.to_string(), rusuario.senha.to_string()).await {
        Ok(u) => {usuario=u},
        Err(e) => println!("Erro = {}",e),
    }
    HttpResponse::Ok().body( format!("{}",usuario.nome))
}

async fn insere_color(rcolor: web::Json<RequestColor>) -> impl Responder {
    println!("/color = {}",&rcolor.color);
    match  postg::insert_color(&rcolor.color).await {
        Ok(_) => {},
        Err(e) => println!("Erro na insert_color: {}", e)
    }
    HttpResponse::Ok().body(rcolor.color.to_string())
}


#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
