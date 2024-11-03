use actix_web::web;

use super::handlers;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(web::scope("/subject").service(handlers::subject_handler::create_subject));
}
