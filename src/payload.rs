use regex::Regex;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"https://github.com/.+/.+/pull/.+").unwrap();
}

#[derive(Deserialize, Debug)]
pub struct Payload {
    comment: Comment,
    repository: Repository,
    issue: Issue,
}

impl Payload {
    pub fn is_pull_request(&self) -> bool {
        REGEX.is_match(&self.issue.html_url)
    }

    pub fn repository_full_name(&self) -> &String {
        &self.repository.full_name
    }

    pub fn issue_number(&self) -> &u64 {
        &self.issue.number
    }

    pub fn comment_body(&self) -> &String
    {
        &self.comment.body
    }
}

#[derive(Deserialize, Debug)]
struct Comment {
    body: String,
}

#[derive(Deserialize, Debug)]
struct Repository {
    full_name: String,
}

#[derive(Deserialize, Debug)]
struct Issue {
    number: u64,
    html_url: String,
}

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


#[cfg(test)]
mod tests {
    use crate::payload::REGEX;

    #[test]
    fn it_will_match_regex() {
        assert!(REGEX.is_match("https://github.com/i_am_an_owner/this-is_repository/pull/3"));
        assert!(REGEX.is_match("https://github.com/aws/das/pull/3"));
    }

    #[test]
    fn it_will_not_match_regex() {
        assert!(!REGEX.is_match("https://github.com/i_am_an_owner/this_is_repository/issues/3"));
        assert!(!REGEX.is_match("https://github.com/aws/das/issues/3"));
    }
}
