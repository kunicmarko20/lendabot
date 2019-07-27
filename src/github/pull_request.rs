#[derive(Deserialize, Debug)]
pub struct PullRequest {
    base: Base,
    head: Head,
}

impl PullRequest {
    pub fn is_release(&self) -> bool {
        &self.base.branch == "master" && &self.head.branch == "development"
    }

    pub fn is_back_merge(&self) -> bool {
        &self.base.branch == "development" && &self.head.branch == "master"
    }

    pub fn is_hotfix(&self) -> bool {
        &self.base.branch == "master" && self.head.branch.starts_with("hotfix")
    }
}

#[derive(Deserialize, Debug)]
struct Base {
    #[serde(rename = "ref")]
    branch: String,
}

#[derive(Deserialize, Debug)]
struct Head {
    #[serde(rename = "ref")]
    branch: String,
}
