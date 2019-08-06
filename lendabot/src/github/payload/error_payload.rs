use crate::github::payload::parts::Error;

#[derive(Deserialize, Debug)]
pub struct ErrorPayload {
    message: String,
    errors: Vec<Error>
}

impl ErrorPayload {
    pub fn to_string(&self) -> String {
        let mut message = "Request to Github failed with error(s): ".to_string();

        for error in &self.errors {
            message = message + &error.message + "; ";
        }

        message
    }
}
