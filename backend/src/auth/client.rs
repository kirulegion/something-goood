use std::env;

use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

pub fn oauth() -> BasicClient {
    //Inserting the client id into the oauth client.
    let client_id = ClientId::new(
        env::var("GOOGLE_CLIENT_ID").expect("Please make sure you provide the client id"),
    );
    //Inserting the client serect into aouth client.
    let client_secret = Some(ClientSecret::new(
        env::var("GOOGLE_CLIENT_SECRET").expect("Please make sure that clinet secret is provided"),
    ));
    //Adding the auth url for google auth.
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("The url provided in the auth url is wrong please check.");
    //Adding the token url.
    let token_url = Some(
        TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
            .expect("The token url might be set wrong check the code."),
    );
    let redirect_url = RedirectUrl::new("http://localhost:8080/auth/google/callback".to_string())
        .expect(
            "Check for the redirction error might be happening due the redirect url being wrong.",
        );

    //Returning the built client.
    BasicClient::new(client_id, client_secret, auth_url, token_url).set_redirect_uri(redirect_url)
}
