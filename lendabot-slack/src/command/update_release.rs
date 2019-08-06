use crate::command::CommandResult;
use lendabot::github::GithubClient;
use regex::Regex;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"\[(?P<ticket>(CARD|LOAN)-\d+)\].*").unwrap();
}

pub(super) struct UpdateRelease;

impl UpdateRelease {
    pub fn execute(
        github_client: GithubClient,
        repository_full_name: &str,
        pull_request_number: u64,
    ) -> CommandResult {
        let pull_request =
            github_client.pull_request_info(repository_full_name, pull_request_number);

        if !pull_request.is_release() {
            return Err("Not a Release Pull Request.".into());
        }

        let mut title = "🤖 Release".to_string();

        let pull_request_commits =
            github_client.pull_request_commits(repository_full_name, pull_request_number);

        for pull_request_commit in pull_request_commits {
            if let Some(captures) = REGEX.captures(&pull_request_commit.commit_message()) {
                if let Some(ticket) = captures.name("ticket") {
                    title = title + " " + ticket.as_str();
                }
            }
        }

        if pull_request.title() == title {
            return Err("No title altering commits founds.".into());
        }

        let body = json!({
            "title": title.as_str(),
        });

        github_client
            .update_pull_request(repository_full_name, pull_request_number, body.to_string())
            .unwrap();

        Ok(())
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
