use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use std::{env, error::Error};

#[derive(Deserialize)]
pub struct OAuthResponse {
    pub access_token: String,
    pub id_token: String,
}

#[derive(Debug, Deserialize)]
pub struct GoogleUserResult {
    pub id: String,           // 사용자의 고유한 Google ID
    pub email: String,        // 사용자의 이메일 주소
    pub verified_email: bool, // 이메일 주소의 인증 여부 (true/false)
    pub name: String,         // 사용자의 전체 이름
    pub given_name: String,   // 사용자의 이름
    pub family_name: String,  // 사용자의 성
    pub picture: String,      // 사용자의 프로필 사진 URL
                              // pub locale: String,       // 사용자의 지역 설정
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct QueryCode {
    pub code: String,
    pub state: String,
}

pub async fn request_token(authorization_code: &str) -> Result<OAuthResponse, Box<dyn Error>> {
    let redirect_url = env::var("REDIRECT_URL").expect("REDIRECT_URL must be set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID must be set");

    let root_url = "https://oauth2.googleapis.com/token";
    let client = Client::new();

    let params = [
        ("grant_type", "authorization_code"),
        ("redirect_uri", redirect_url.as_str()),
        ("client_id", client_id.as_str()),
        ("code", authorization_code),
        ("client_secret", client_secret.as_str()),
    ];
    let response = client.post(root_url).form(&params).send().await?;

    if response.status().is_success() {
        let oauth_response = response.json::<OAuthResponse>().await?;
        Ok(oauth_response)
    } else {
        let message = "An error occurred while trying to retrieve access token.";
        Err(From::from(message))
    }
}

pub async fn get_google_user(
    access_token: &str,
    id_token: &str,
) -> Result<GoogleUserResult, Box<dyn Error>> {
    let client = Client::new();
    let mut url = Url::parse("https://www.googleapis.com/oauth2/v2/userinfo").unwrap();
    url.query_pairs_mut().append_pair("alt", "json");
    url.query_pairs_mut()
        .append_pair("access_token", access_token);

    let response = client.get(url).bearer_auth(id_token).send().await?;

    if response.status().is_success() {
        let user_info = response.json::<GoogleUserResult>().await?;
        println!("{:?}", user_info);
        Ok(user_info)
    } else {
        let message = "An error occurred while trying to retrieve user information";
        Err(From::from(message))
    }
}
