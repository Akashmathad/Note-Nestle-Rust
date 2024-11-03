use actix_web::web::{self};

use super::handlers;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/subject")
            .service(handlers::subject_handler::create_subject)
            .service(handlers::subject_handler::create_unit)
            .service(handlers::subject_handler::delete_unit)
            .service(handlers::subject_handler::delete_subject)
            .service(handlers::subject_handler::get_units)
            .service(handlers::subject_handler::get_subjects),
    );
}
