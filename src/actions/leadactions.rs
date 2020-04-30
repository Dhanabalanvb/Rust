use actix_web::{ Responder, get, HttpRequest};
use crate::models::lead::*;

#[get("/leadactions/getlead")]
pub async fn get_lead(_req:HttpRequest) -> impl Responder{
    LeadModel::new(1, "Sample Lead".to_string())
}