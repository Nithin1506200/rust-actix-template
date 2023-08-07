use serde::Deserialize;
#[derive(Deserialize)]
pub struct TestUser {
    pub user_id: String,
    pub name: String,
}
