// sends push notifications to unique streams
use crate::api::nonkyc::NonKycClient;
use reqwest::{Error, Response};

const BASE: &'static str = "https://ntfy.sh/";
const UNIQUE_PREFIX: &'static str = "1ef23c2aab8f995a7eedaae4355cd02ce86e348f39daedaa12_";
// in the event of "stream pollution" this must be changed
// to listen to the stream simply subscribe to it with either of the
// supported prefixes
// you can subscribe to multiple streams!

pub struct NftyClient {
    pub client: reqwest::Client,
}

impl NftyClient {
    pub async fn submit(&self, message: String, suffix: &str) -> Result<Response, Error> {
        let stream = format!("{}{}", UNIQUE_PREFIX, suffix);
        let url = format!("{}{}", BASE, stream);
        self.client.post(url).body(message).send().await
    }
}

#[cfg(test)]
mod tests {
    use super::NftyClient;
    #[tokio::test]
    async fn test_ntfy_submit() {
        let client = NftyClient {
            client: reqwest::Client::new(),
        };
        let result = client
            .submit("hello, this is a test".to_string(), "test-stream")
            .await;
        result.unwrap();
    }
}
