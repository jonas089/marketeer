use marketeer::api::nonkyc::{auth::authentication, types::Balances};
use reqwest::Client;

pub async fn get_non_zero_balances() -> Balances {
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
    let balances: Balances = Balances {
        balances: serde_json::from_str(&response).unwrap(),
    };
    balances
}
