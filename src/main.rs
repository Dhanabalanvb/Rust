extern crate actix_web;

mod models;
mod actions;

use actix_web::{ App,  HttpServer };
use actions::leadactions::*;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_lead)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}