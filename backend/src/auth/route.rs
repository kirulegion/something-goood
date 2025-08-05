use actix_web::{HttpRequest, HttpResponse, cookie::Cookie, get, http::header, web};
use oauth2::{
    AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope, basic::BasicClient,
    reqwest::async_http_client,
};
use serde_json::json;
use std::collections::HashMap;
use crate::oauth;

use crate::auth::{client::OauthProvider};

#[get("/auth/{provider}")]
pub async fn login(provider : web::Path<OauthProvider>) -> HttpResponse {
    let oauth_client = oauth(provider.into_inner());
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csfr_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    let cookie1: Cookie = Cookie::build("oauth_pkce_verifier", pkce_verifier.secret().clone())
        .path("/")
        .http_only(true)
        .secure(false)
        .finish();

    let cookie2: Cookie = Cookie::build("oauth_csrf", csfr_token.secret().clone())
        .path("/")
        .http_only(true)
        .secure(false)
        .finish();
    HttpResponse::Found()
        .insert_header((header::LOCATION, auth_url.to_string()))
        .cookie(cookie1)
        .cookie(cookie2)
        .finish()
}

#[get("/auth/{provider}/callback")]
pub async fn callback(
    request: HttpRequest,
    params: web::Query<HashMap<String, String>>,
    oauth_client: web::Data<BasicClient>,
) -> HttpResponse {
    //Extracting the code for the exchanging the acess-token.
    let code = match params.get("code") {
        Some(code) => code,
        None => {
            return HttpResponse::BadRequest()
                .json(json!({"error" : "Missing authorization code"}));
        }
    };

    //Extracting the state from the url to valid that isn't a CSRF attack.
    let state_from_query = match params.get("state") {
        Some(state) => state,
        None => return HttpResponse::BadRequest().json(json!({"Error" : "MIssing the state"})),
    };

    let pkce_verifier_from_query = match request.cookie("oauth_pkce_verifier") {
        Some(cookie) => cookie.value().to_string(),
        None => return HttpResponse::BadRequest().json(json!({"Error" : "MIssing the Cookie"})),
    };

    //Extracting the state from the cookies to valid that isn't a CSRF attack.
    let state_from_cookie = match request.cookie("oauth_csrf") {
        Some(cookie) => cookie.value().to_string(),
        None => return HttpResponse::BadRequest().json(json!({"Error" : "MIssing the Cookie"})),
    };
    //Checking for CSRF
    if *state_from_query != state_from_cookie {
        return HttpResponse::Forbidden().body("State mismatch (Possible CSRF)");
    }

    //Exchange Code for token.
    let token_result = oauth_client
        .exchange_code(AuthorizationCode::new(code.to_string()))
        .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier_from_query))
        .request_async(async_http_client)
        .await;

    let token = match token_result {
        Ok(token) => token,
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(format!("Token exchange failed : {}", err));
        }
    };

    HttpResponse::Ok().body(format!("OAuth Success! Token: {:#?}", token))
}
