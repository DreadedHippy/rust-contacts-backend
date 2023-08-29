
use actix_cors::Cors;
use actix_governor::{GovernorConfigBuilder, Governor};
use actix_web::{App, HttpServer, get, Responder, HttpResponse};
// use std::io::{Error,ErrorKind};
use rust_contacts_backend::routes;
use serde::Serialize;


#[derive(Serialize)]
struct MyObj {
    message: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    let res = MyObj{message: "Hello World".to_string()};
    HttpResponse::Ok().json(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Actix backend server");

    // Rate limit configuration
    let governor_config = GovernorConfigBuilder::default()
        .per_second(3)
        .burst_size(20)
        .finish()
        .unwrap();


    // HTTP Server start
    HttpServer::new(move ||{
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method()
            )
            .wrap(Governor::new(&governor_config))
            .service(hello)
            .service(routes::list_contacts)
            .service(routes::create_contact)
            .service(routes::get_contact)
            .service(routes::delete_contact)
            .service(routes::edit_contact)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}