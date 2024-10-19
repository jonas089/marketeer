use hex::ToHex;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};
type HmacSha256 = Hmac<Sha256>;

pub struct AuthData {
    pub signature: String,
    pub api_key: String,
    pub nonce: String,
}

pub fn authentication(payload: &str) -> AuthData {
    let public_key = env::var("NON_KYC_PUBLIC_KEY").expect("Missing public key for nonkyc api");
    let secret_key = env::var("NON_KYC_SECRET_KEY").expect("Missing secret key for nonkyc api");

    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();
    let message = format!("{}{}{}", public_key, payload, nonce);
    let mut mac = HmacSha256::new_from_slice(secret_key.as_bytes()).unwrap();
    mac.update(message.as_bytes());
    let result = mac.finalize();
    let signature_hex = result.into_bytes().encode_hex();
    println!("{:?}", signature_hex);
    AuthData {
        signature: signature_hex,
        api_key: public_key.to_string(),
        nonce,
    }
}

#[cfg(test)]
mod test {
    use super::authentication;
    use crate::api::nonkyc::types::Balances;
    use reqwest::Client;
    #[tokio::test]
    async fn test_api_auth() {
        let url = "https://api.nonkyc.io/api/v2/balances";
        let auth = authentication(&url);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("X-API-KEY", auth.api_key.parse().unwrap());
        headers.insert("X-API-NONCE", auth.nonce.parse().unwrap());
        headers.insert("X-API-SIGN", auth.signature.parse().unwrap());
        headers.insert("Content-Type", "application/json".parse().unwrap());
        let client = Client::new();
        let response = client
            .get(url)
            .headers(headers)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        println!("Response: {:?}", &response);
        let balances: Balances = Balances {
            balances: serde_json::from_str(&response).unwrap(),
        };
        println!("Non-zero Balances: {:?}", &balances.get_non_zero());
    }
}
