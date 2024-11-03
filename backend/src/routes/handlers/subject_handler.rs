use actix_web::delete;
use actix_web::{get, post, web, Responder};
use entity::{subject, unit};
use sea_orm::{ActiveModelTrait, EntityTrait};
use sea_orm::{ColumnTrait, QueryOrder};
use sea_orm::{QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serde_json::to_string;

use uuid::Uuid;

use crate::utils::{api_response, app_state::AppState};

#[derive(Serialize, Deserialize)]
struct SubjectModel {
    name: String,
    branch: String,
}

#[derive(Serialize, Deserialize)]
struct UnitModel {
    name: String,
    subject_Id: String,
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

#[post("/create_unit")]
pub async fn create_unit(
    app_state: web::Data<AppState>,
    unit_json: web::Json<UnitModel>,
) -> impl Responder {
    let unit = entity::unit::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(unit_json.name.clone()),
        subject_id: Set(Uuid::parse_str(&unit_json.subject_Id).unwrap()),
    }
    .insert(&app_state.db)
    .await
    .unwrap();

    api_response::ApiResponse::new(200, to_string(&unit).unwrap())
}

#[delete("/delete_unit/{unit_Id}")]
pub async fn delete_unit(
    unit_id: web::Path<Uuid>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    entity::unit::Entity::delete_by_id(*unit_id)
        .exec(&app_state.db)
        .await
        .unwrap();

    api_response::ApiResponse::new(200, "Deleted".to_string())
}

#[delete("/delete_subject/{subject_id}")]
pub async fn delete_subject(
    subject_id: web::Path<Uuid>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    entity::subject::Entity::delete_by_id(*subject_id)
        .exec(&app_state.db)
        .await
        .unwrap();
    api_response::ApiResponse::new(200, "Deleted".to_string())
}

#[get("/units/{subject_id}")]
pub async fn get_units(
    subject_id: web::Path<Uuid>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let units = entity::unit::Entity::find()
        .filter(unit::Column::SubjectId.eq(*subject_id))
        .all(&app_state.db)
        .await
        .unwrap();

    api_response::ApiResponse::new(200, to_string(&units).unwrap())
}

#[get("/subjects/{branch}")]
pub async fn get_subjects(
    branch: web::Path<String>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let subjects = entity::subject::Entity::find()
        .filter(subject::Column::Branch.eq(branch.clone()))
        .order_by_asc(subject::Column::Name)
        .all(&app_state.db)
        .await
        .unwrap();

    api_response::ApiResponse::new(200, to_string(&subjects).unwrap())
}
