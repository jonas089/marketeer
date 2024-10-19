use crate::api::nonkyc::secret::{PUBLIC_KEY, SECRET};
use hex::ToHex;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
type HmacSha256 = Hmac<Sha256>;

pub struct AuthData {
    pub signature: String,
    pub api_key: String,
    pub nonce: String,
}

pub fn authentication(payload: &str) -> AuthData {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();
    let message = format!("{}{}{}", PUBLIC_KEY, payload, nonce);
    let mut mac = HmacSha256::new_from_slice(SECRET.as_bytes()).unwrap();
    mac.update(message.as_bytes());
    let result = mac.finalize();
    let signature_hex = result.into_bytes().encode_hex();
    println!("{:?}", signature_hex);
    AuthData {
        signature: signature_hex,
        api_key: PUBLIC_KEY.to_string(),
        nonce,
    }
}

#[cfg(test)]
mod test {
    use reqwest::Client;

    use super::authentication;
    #[tokio::test]
    async fn test_api_auth() {
        let url = "https://api.nonkyc.io/api/v2/balances";
        let payload = "https://api.nonkyc.io/api/v2/balances";
        let auth = authentication(payload);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("X-API-KEY", auth.api_key.parse().unwrap());
        headers.insert("X-API-NONCE", auth.nonce.parse().unwrap());
        headers.insert("X-API-SIGN", auth.signature.parse().unwrap());
        headers.insert("Content-Type", "application/json".parse().unwrap());
        let client = Client::new();
        let response = client.get(url).headers(headers).send().await;
        println!("Balances: {:?}", &response.unwrap().text().await.unwrap());
    }
}