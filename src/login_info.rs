use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginInfo {
    id: String,
    password: String,
}

impl LoginInfo {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn password(&self) -> &String {
        &self.password
    }
}
