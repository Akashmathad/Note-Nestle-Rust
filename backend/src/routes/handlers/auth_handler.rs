use actix_web::{web, Responder};
use serde::{Deserialize, Serialize};

use crate::utils::app_state::{self, AppState};

#[derive(Serialize, Deserialize)]
struct RegisterModel {
    name: String,
    email: String,
    password: String,
}
#[derive(Serialize, Deserialize)]
struct LoginModel {
    email: String,
    password: String,
}

// #[post("/register")]

// pub async fn register(
//     app_state: web::Data<AppState>,
//     register_json: web::Json<RegisterModel>,
// ) -> impl Responder {
// }
