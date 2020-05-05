use actix_web::{ web,Responder, get, HttpRequest};
use crate::models::lead::*;

#[get("/leadactions/getlead")]
pub async fn get_lead(_req:HttpRequest) -> impl Responder{
    LeadModel::new(1, "Sample Lead".to_string())
}

#[get("/leadactions/getlead2")]
pub async fn get_lead2(_req:HttpRequest) -> impl Responder{
    LeadModel::new(2, "Sample Lead2".to_string())
}

pub fn config(cfg:&mut web::ServiceConfig){
    cfg.service(get_lead);
    cfg.service(get_lead2);
}