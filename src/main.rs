use actix_web::{HttpServer, App};
use actix_web::{get, post, web, HttpResponse, Responder};
use data_access::postg;
use http::RequestColor;

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
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
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
