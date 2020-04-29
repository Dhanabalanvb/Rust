use actix_web::{ HttpResponse, Responder};
pub mod Lead;

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello worlds : index1 !")
}