use actix_web::{web, App, HttpResponse, HttpServer, Responder};
mod models;



#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/",web::get().to(models::index))
            .route("/ind2",web::get().to(models::Lead::index2))
        
           // .route("/", web::get().to(index))
            //.route("/again", web::get().to(index2))
    })
    .bind("192.168.43.25:8088")?
    .run()
    .await
}