use actix_web::web::{self};

use super::handlers;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/feedback")
            .service(handlers::feedback_handler::submit_feedback)
            .service(handlers::feedback_handler::delete_feedback)
            .service(handlers::feedback_handler::get_feedbacks),
    );
}
