use crate::utils::jwt::encode_jwt;
use crate::utils::{api_response, app_state::AppState};
use actix_web::{post, web, Responder};
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::Condition;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use sha256::digest;
use uuid::Uuid;

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

#[derive(Serialize, Deserialize)]
struct ApiBody<T> {
    status: String,
    token: String,
    data: T,
}

#[derive(Serialize, Deserialize)]
struct LoginBody {
    id: Uuid,
    name: String,
    email: String,
}

#[post("/register")]

pub async fn register(
    app_state: web::Data<AppState>,
    register_json: web::Json<RegisterModel>,
) -> impl Responder {
    let user_model = entity::user::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(register_json.name.clone()),
        email: Set(register_json.email.clone()),
        password: Set(digest(&register_json.password)),
    }
    .insert(&app_state.db)
    .await
    .unwrap();

    api_response::ApiResponse::new(200, format!("{}", user_model.id))
}

#[post("/login")]
pub async fn login(
    app_state: web::Data<AppState>,
    login_json: web::Json<LoginModel>,
) -> impl Responder {
    let user = entity::user::Entity::find()
        .filter(
            Condition::all()
                .add(entity::user::Column::Email.eq(&login_json.email))
                .add(entity::user::Column::Password.eq(digest(&login_json.password))),
        )
        .one(&app_state.db)
        .await
        .unwrap();

    if user.is_none() {
        return api_response::ApiResponse::new(401, "User not found".to_string());
    }

    let user_data = user.unwrap();
    let token = encode_jwt(user_data.email.clone(), user_data.id).unwrap();

    let login_body: LoginBody = LoginBody {
        id: user_data.id,
        name: user_data.name,
        email: user_data.email,
    };

    let body: ApiBody<LoginBody> = ApiBody {
        status: "Success".to_string(),
        token,
        data: login_body,
    };

    api_response::ApiResponse::new(200, to_string(&body).unwrap())
}
