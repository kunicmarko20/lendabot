#[derive(Deserialize, Debug)]
pub struct Issue {
    title: String,
    number: u64,
    html_url: String,
}

impl Issue {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn number(&self) -> u64 {
        self.number
    }

    pub fn html_url(&self) -> &str {
        &self.html_url
    }
}
