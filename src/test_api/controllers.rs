use actix_web::{get, post, web, Result};

use crate::test_api::model::TestUser;

#[get("{user_id}/{name}")]
pub async fn get_user(path: web::Path<(String, String)>) -> Result<String> {
    let (user_id, name) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}!", name, user_id))
}

#[post("/post")]
pub async fn post_user(path: web::Json<TestUser>) -> Result<String> {
    Ok(format!("hello!, {}", path.name))
}
