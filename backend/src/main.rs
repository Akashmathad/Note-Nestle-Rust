use actix_web::{middleware::Logger, web, App, HttpServer};
use sea_orm::{Database, DatabaseConnection};
use utils::app_state::AppState;

mod routes;
mod utils;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv::dotenv().ok();
    env_logger::init();

    let port = utils::constants::PORT.clone();
    let address = (*utils::constants::ADDRESS).clone();
    let database_url = utils::constants::DATABASE_URL.clone();

    let db: DatabaseConnection = Database::connect(database_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .wrap(Logger::default())
            .configure(routes::subject_routes::config)
            .configure(routes::auth_routes::config)
            .configure(routes::feedback_routes::config)
    })
    .bind((address, port))?
    .run()
    .await
}
