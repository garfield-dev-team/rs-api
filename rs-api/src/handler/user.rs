use actix_web::{web, HttpResponse, Responder};
use validator::{Validate, ValidationErrors};

use crate::model::user::UserRequest;

#[actix_web::post("/user/create")]
pub async fn create_user(user: web::Json<UserRequest>) -> impl Responder {
    match user.validate() {
        Ok(_) => HttpResponse::Ok().body(format!("User {} created!", user.name)),
        Err(e) => HttpResponse::BadRequest().body(format!("Validation error: {:?}", e)),
    }
}
