use actix_web::{post, web, Responder};
use sea_orm::ActiveModelTrait;
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use uuid::Uuid;

use crate::utils::{
    api_response,
    app_state::{self, AppState},
};

#[derive(Serialize, Deserialize)]
struct SubjectModel {
    name: String,
    branch: String,
}

#[post("/create_subject")]
pub async fn create_subject(
    app_state: web::Data<AppState>,
    subject_json: web::Json<SubjectModel>,
) -> impl Responder {
    let subject = entity::subject::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(subject_json.name.clone()),
        branch: Set(subject_json.branch.clone()),
    }
    .insert(&app_state.db)
    .await
    .unwrap();

    api_response::ApiResponse::new(200, to_string(&subject).unwrap())
}
