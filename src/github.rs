use std::net::SocketAddr;

use anyhow::Ok;
use oauth2::*;
use url::Url;

#[derive(serde::Deserialize)]
pub struct ReceivedCode {
    pub code: AuthorizationCode,
    pub state: State,
}

pub async fn get_link() -> String {
    let auth_url = Url::parse("https://github.com/login/oauth/authorize").unwrap();
    let token_url = Url::parse("https://github.com/login/oauth/access_token").unwrap();

    let mut client = Client::new(include_str!("resources\\id.txt").to_string(), auth_url, token_url);
    client.set_client_secret(include_str!("resources\\secret.txt").to_string());
    client.set_redirect_url(Url::parse("http://reynard.me:4000").unwrap());
    client.add_scope("repo");

    let state = State::new_random();
    let auth_url = client.authorize_url(&state);
    auth_url.to_string()
}

pub async fn get_token() -> String {
    let reqwest_client = reqwest::Client::new();

    let auth_url = Url::parse("https://github.com/login/oauth/authorize").unwrap();
    let token_url = Url::parse("https://github.com/login/oauth/access_token").unwrap();

    let mut client = Client::new(include_str!("resources\\id.txt").to_string(), auth_url, token_url);
    client.set_client_secret(include_str!("resources\\secret.txt").to_string());
    client.set_redirect_url(Url::parse("http://reynard.me:4000").unwrap());
    client.add_scope("repo");

    let state = State::new_random();
    let auth_url = client.authorize_url(&state);

    println!("Browse to: {}", auth_url);




    listen_for_code();

    String::new()

    // let received = listen_for_code().await.unwrap();

    // if received.state != state {
    //     panic!("CSRF token mismatch :(");
    // }

    // let token = client
    //     .exchange_code(received.code)
    //     .with_client(&reqwest_client)
    //     .execute::<StandardToken>()
    //     .await.unwrap();

    // token.access_token().to_string()
}

async fn listen_for_code() {
    
}