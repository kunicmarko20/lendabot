use regex::Regex;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"https://github.com/(\w+)/(\w+)/pull/(\d+)").unwrap();
}

#[derive(Deserialize, Debug)]
pub struct Payload {
    comment: Comment,
    repository: Repository,
}

impl Payload {
    pub fn is_pull_request(&self) -> bool {
        REGEX.is_match(&self.comment.html_url)
    }

    pub fn repository_full_name(&self) -> &String
    {
        &self.repository.full_name
    }

    pub fn comment_body(&self) -> &String
    {
        &self.comment.body
    }
}

#[derive(Deserialize, Debug)]
struct Comment {
    html_url: String,
    body: String,
}

#[derive(Deserialize, Debug)]
struct Repository {
    full_name: String,
}

#[cfg(test)]
mod tests {
    use crate::REGEX;

    #[test]
    fn it_will_match_regex() {
        assert!(REGEX.is_match("https://github.com/i_am_an_owner/this_is_repository/pull/3"));
        assert!(REGEX.is_match("https://github.com/aws/das/pull/3"));
    }

    #[test]
    fn it_will_not_match_regex() {
        assert!(!REGEX.is_match("https://github.com/i_am_an_owner/this_is_repository/issues/3"));
        assert!(!REGEX.is_match("https://github.com/aws/das/issues/3"));
    }
}
