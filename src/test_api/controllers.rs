use actix_web::{get, post, web, Result};
use serde::Deserialize;

#[get("{user_id}/{name}")]
pub async fn get_user(path: web::Path<(String, String)>) -> Result<String> {
    let (user_id, name) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}!", name, user_id))
}
#[derive(Deserialize)]
pub struct UserInput {
    user_name: String,
}
#[post("/post")]
pub async fn post_user(path: web::Json<UserInput>) -> Result<String> {
    Ok(format!("hello!{}", path.user_name))
}
