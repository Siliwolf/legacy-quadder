use cookie::Cookie;

use crate::*;

pub fn store_token(token: String) {
    let mut cookie = Cookie::build(TOKEN_KEY, token)
        .secure(true)
        .http_only(true)
        .finish();

    cookie.make_permanent();
}

pub fn token_exists() -> bool {
    Cookie::parse(TOKEN_KEY).is_ok()
}

pub fn get_token() -> String {
    Cookie::parse(TOKEN_KEY).unwrap().to_string()
}