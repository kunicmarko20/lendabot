use super::GITHUB_CLIENT;
use regex::Regex;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"\[(?P<ticket>(CARD|LOAN)-\d+)\].*").unwrap();
}

pub(super) struct UpdateRelease;

impl UpdateRelease {
    pub fn execute(repository_full_name: &String, pull_request_number: &u64) {
        let pull_request = GITHUB_CLIENT.pull_request_info(
            repository_full_name,
            pull_request_number,
        );

        if !pull_request.is_release() {
            return;
        }

        let mut title = "🤖 Release".to_string();

        let pull_request_commits = GITHUB_CLIENT.pull_request_commits(
            repository_full_name,
            pull_request_number,
        );

        for pull_request_commit in pull_request_commits {
            if let Some(captures) = REGEX.captures(&pull_request_commit.commit_message()) {
                if let Some(ticket) = captures.name("ticket") {
                    title = title + " " + ticket.as_str();
                }
            }
        }

        let body = json!({
            "title": title.as_str(),
        });

        GITHUB_CLIENT.update_pull_request(
            repository_full_name,
            pull_request_number,
            body.to_string(),
        ).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::REGEX;

    #[test]
    fn it_will_match_regex() {
        let captures = REGEX.captures("[CARD-321] this is a pr").unwrap();

        assert!(captures.name("ticket").unwrap().as_str() == "CARD-321");

        let captures = REGEX.captures("[LOAN-321] this is a pr").unwrap();

        assert!(captures.name("ticket").unwrap().as_str() == "LOAN-321");
    }
}
