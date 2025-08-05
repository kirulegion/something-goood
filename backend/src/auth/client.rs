use std::{env, str::FromStr};

use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl, basic::BasicClient};
use serde::Deserialize;


#[derive(Debug , Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OauthProvider {
    Google,
    GitHub,
}

impl FromStr for OauthProvider {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "google" => {
                Ok(OauthProvider::Google)
            },
            "github" => {
                Ok(OauthProvider::GitHub)
            },
            _ => Err(()),
        }
    }
}

pub fn oauth(provider: OauthProvider) -> BasicClient {
    let client_id;
    let client_secret;
    let auth_url;
    let redirect_url;
    let token_url;

    match provider {
        OauthProvider::Google => {
            client_id = ClientId::new(
                env::var("GOOGLE_CLIENT_ID").expect("Please make sure you provide the client id"),
            );

            client_secret = Some(ClientSecret::new(
                env::var("GOOGLE_CLIENT_SECRET")
                    .expect("Please make sure that clinet secret is provided"),
            ));

            auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
                .expect("The url provided in the auth url is wrong please check.");

            token_url = Some(
                TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
                    .expect("The token url might be set wrong check the code."),
            );

            redirect_url = RedirectUrl::new("http://localhost:8080/auth/google/callback".to_string())
        .expect(
            "Check for the redirction error might be happening due the redirect url being wrong.",
        );
        }

        OauthProvider::GitHub => {
            client_id = ClientId::new(
                env::var("GITHUB_CLIENT_ID").expect("Please make sure you provide the client id"),
            );

            client_secret = Some(ClientSecret::new(
                env::var("GITHUB_CLIENT_SECRET")
                    .expect("Please make sure that clinet secret is provided"),
            ));

            auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
                .expect("The url provided in the auth url is wrong please check.");

            token_url = Some(
                TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
                    .expect("The token url might be set wrong check the code."),
            );

            redirect_url = RedirectUrl::new("http://localhost:8080/auth/github/callback".to_string())
        .expect(
            "Check for the redirction error might be happening due the redirect url being wrong.",
        );
        }
    }

    BasicClient::new(client_id, client_secret, auth_url, token_url).set_redirect_uri(redirect_url)
}
