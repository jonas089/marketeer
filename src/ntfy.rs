// sends push notifications to unique streams
use reqwest::{Error, Response};
pub mod constants;
use constants::{BASE, UNIQUE_PREFIX};
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
