use actix_web::{delete, get};
use actix_web::{post, web, Responder};
use sea_orm::ActiveModelTrait;
use sea_orm::EntityTrait;
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use uuid::Uuid;

use crate::utils::api_response;
use crate::utils::app_state::AppState;

#[derive(Serialize, Deserialize)]
struct FeedbackModel {
    title: String,
    message: String,
    user_id: Uuid,
}

#[get("")]
pub async fn get_feedbacks(app_state: web::Data<AppState>) -> impl Responder {
    let feedbacks = entity::feedback::Entity::find()
        .all(&app_state.db)
        .await
        .unwrap();

    api_response::ApiResponse::new(200, to_string(&feedbacks).unwrap())
}

#[post("/submit")]
pub async fn submit_feedback(
    app_state: web::Data<AppState>,
    feedback_json: web::Json<FeedbackModel>,
) -> impl Responder {
    entity::feedback::ActiveModel {
        id: Set(Uuid::new_v4()),
        title: Set(feedback_json.title.clone()),
        message: Set(feedback_json.message.clone()),
        user_id: Set(feedback_json.user_id),
    }
    .insert(&app_state.db)
    .await
    .unwrap();

    api_response::ApiResponse::new(200, "Feedback sumbitted".to_string())
}

#[delete("/delete/{feedback_id}")]
pub async fn delete_feedback(
    app_state: web::Data<AppState>,
    feedback_id: web::Path<Uuid>,
) -> impl Responder {
    entity::feedback::Entity::delete_by_id(*feedback_id)
        .exec(&app_state.db)
        .await
        .unwrap();

    api_response::ApiResponse::new(200, "Feedback deleted".to_string())
}
