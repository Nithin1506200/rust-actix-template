mod controllers;
mod model;

use actix_web::{web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/test").service(
            web::resource("/test")
                .route(web::get().to(|| async { HttpResponse::Ok().body("this is get") }))
                .route(web::head().to(HttpResponse::MethodNotAllowed)),
        ),
    )
    .service(
        web::scope("/nested")
            .service(controllers::get_user)
            .service(controllers::post_user),
    );
}
