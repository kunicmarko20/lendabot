use lendabot::github::payload::{Comment, Issue, Repository};
use regex::Regex;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"https://github.com/.+/.+/pull/.+").unwrap();
}

#[derive(Deserialize, Debug)]
pub struct IssueCommentEventPayload {
    comment: Comment,
    repository: Repository,
    issue: Issue,
}

impl IssueCommentEventPayload {
    pub fn is_pull_request(&self) -> bool {
        REGEX.is_match(&self.issue.html_url())
    }

    pub fn repository_full_name(&self) -> &str {
        &self.repository.full_name()
    }

    pub fn issue_number(&self) -> u64 {
        self.issue.number()
    }

    pub fn issue_title(&self) -> &str {
        &self.issue.title()
    }

    pub fn comment_body(&self) -> &str {
        &self.comment.body()
    }
}

#[cfg(test)]
mod tests {
    use super::REGEX;

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
