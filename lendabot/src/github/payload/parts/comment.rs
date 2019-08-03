#[derive(Deserialize, Debug)]
pub struct Comment {
    body: String,
}

impl Comment {
    pub fn body(&self) -> &str {
        &self.body
    }
}
