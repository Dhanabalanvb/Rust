use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use futures::future::{ready, Ready};

#[derive(Debug, Deserialize, Serialize)]
pub struct LeadModel {
    id:i32,
    name:String
}

impl LeadModel {
    pub fn new(id: i32, name: String) -> Self { Self { id, name } }
}

impl Responder for LeadModel{
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }

}