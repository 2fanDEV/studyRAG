use std::collections::HashMap;

use actix_web::{post, web::Query, HttpResponse};
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GitHubCode {
    code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GitHubTokenResponse {
    access_token: String,
    token_type: String,
    scope: String,
}

#[post("auth")]
pub async fn exchange_token(code: Query<GitHubCode>) -> HttpResponse {
    debug!("{code:?}");
    let client = reqwest::Client::new();
    let client_id = match std::env::var("GITHUB_CLIENT_ID") {
        Ok(github_client_id) => github_client_id,
        Err(error) => match dotenv::var("GITHUB_CLIENT_ID") {
            Ok(github_client_id) => github_client_id,
            Err(_) => panic!("Failed to retrieve gh client id out of env/.env"),
        },
    };
    let client_secret = match std::env::var("GITHUB_CLIENT_SECRET") {
        Ok(github_client_secret) => github_client_secret,
        Err(error) => match dotenv::var("GITHUB_CLIENT_SECRET") {
            Ok(github_client_secret) => github_client_secret,
            Err(_) => panic!("Failed to retrieve gh client secret out of env/.env"),
        },
    };

    let redirect_url = match std::env::var("GITHUB_REDIRECT_URL") {
        Ok(gh_redirect_url) => gh_redirect_url,
        Err(error) => match dotenv::var("GITHUB_REDIRECT_URL") {
            Ok(gh_redirect_url) => gh_redirect_url,
            Err(_) => panic!("Failed to retrieve gh client secret out of env/.env"),
        },
    };
    #[derive(Debug, Serialize)]
    struct OAuthParams {
        client_id: String,
        client_secret: String,
        code: String,
        redirect_uri: String,
    }

    let params = OAuthParams {
        client_id,
        client_secret,
        code: code.into_inner().code,
        redirect_uri: redirect_url,
    };

    let response = client
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .form(&params)
        .send()
        .await
        .unwrap();

    debug!("{response:?}");

    HttpResponse::Ok().json(response.json::<GitHubTokenResponse>().await.unwrap())
}
