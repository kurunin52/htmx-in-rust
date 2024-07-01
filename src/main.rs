use std::env;
use std::net::Ipv4Addr;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use tera::{Context, Tera};

#[get("/")]
async fn hello() -> impl Responder {
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    let ctx = Context::new();
    let t = tera.render("index.html", &ctx).unwrap();
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(t)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port_key = "PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 8080,
    };

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((Ipv4Addr::UNSPECIFIED, port))?
    .run()
    .await
}