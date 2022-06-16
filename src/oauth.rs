//! Get an OAuth2 token from Fitbit without user interaction using the
//! [OAuth 2.0 Authorization Code Grant](https://tools.ietf.org/html/rfc6749#section-4.1) flow.
//!
//! This capture method was inspired by
//! [oauth2-rs](https://github.com/ramosbugs/oauth2-rs/tree/master/examples).

use std::{
    cell::{Ref, RefCell},
    ops::Deref,
    path::{Path, PathBuf},
};

use oauth2::{
    basic::{BasicClient, BasicTokenType},
    reqwest::async_http_client,
    url::Url,
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    RefreshToken, Scope, StandardTokenResponse, TokenResponse, TokenUrl,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

use crate::Result;

type Token = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

pub(crate) struct Auth {
    client_id: String,
    client_secret: String,
    cache_path: Option<PathBuf>,
    token: RefCell<Option<Token>>,
}

impl Auth {
    pub(crate) fn new(
        client_id: String,
        client_secret: String,
        cache_path: Option<PathBuf>,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            cache_path,
            token: RefCell::new(None),
        }
    }
    pub(crate) fn with_cache<P>(&mut self, path: P) -> &mut Self
    where
        P: Into<PathBuf>,
    {
        self.cache_path = Some(path.into());
        self
    }

    pub(crate) async fn auth_interactive(&self) {
        if self.token.borrow().is_some() {
            return;
        }

        if let Some(cache_path) = &self.cache_path {
            if let Ok(token) = read_auth_token(cache_path) {
                *self.token.borrow_mut() = Some(token);
                return;
            }
        }

        let token = fetch_token(&self.client_id, &self.client_secret).await;
        self.save_token(token);
    }

    pub(crate) async fn refresh_token(&self) -> Option<impl Deref<Target = str> + '_> {
        let refresh_token = match self.get_refresh_token() {
            Some(refresh_token) => refresh_token,
            None => return None,
        };

        let client = client(&self.client_id, &self.client_secret);
        let new_token = match client
            .exchange_refresh_token(&refresh_token)
            .request_async(async_http_client)
            .await
        {
            Ok(t) => t,
            Err(e) => {
                eprintln!("OAuth2: {}", e);
                eprintln!("Invalid refresh token. Clearing.");
                if let Some(cache_path) = &self.cache_path {
                    clear_auth_token(cache_path).unwrap();
                }
                std::process::exit(1);
            }
        };
        self.save_token(new_token);
        self.get_token()
    }

    pub(crate) fn get_token(&self) -> Option<impl Deref<Target = str> + '_> {
        let token = self.token.borrow();
        if token.is_some() {
            Some(Ref::map(token, |o| {
                o.as_ref().unwrap().access_token().secret().as_str()
            }))
        } else {
            None
        }
    }

    fn get_refresh_token(&self) -> Option<RefreshToken> {
        self.token
            .borrow()
            .as_ref()
            .and_then(|t| t.refresh_token())
            .map(|r| r.to_owned())
    }

    fn save_token(&self, token: Token) {
        if let Some(cache_path) = &self.cache_path {
            write_auth_token(&token, cache_path).expect("Couldn't write auth token");
        }
        *self.token.borrow_mut() = Some(token);
    }
}

fn client(client_id: &str, client_secret: &str) -> BasicClient {
    BasicClient::new(
        ClientId::new(client_id.to_owned()),
        Some(ClientSecret::new(client_secret.to_owned())),
        AuthUrl::new("https://www.fitbit.com/oauth2/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://api.fitbit.com/oauth2/token".to_string()).unwrap()),
    )
}

/// Get a token via the OAuth 2.0 Implicit Grant Flow
async fn fetch_token(client_id: &str, client_secret: &str) -> Token {
    let client = client(client_id, client_secret);

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("activity".to_string()))
        .add_scope(Scope::new("heartrate".to_string()))
        .add_scope(Scope::new("location".to_string()))
        .add_scope(Scope::new("nutrition".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("settings".to_string()))
        .add_scope(Scope::new("sleep".to_string()))
        .add_scope(Scope::new("social".to_string()))
        .add_scope(Scope::new("weight".to_string()))
        .url();

    println!("Open the following in your browser: {}", authorize_url);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    if let Ok((mut stream, _)) = listener.accept().await {
        let code;
        let state;
        {
            let mut reader = BufReader::new(&mut stream);

            let mut request_line = String::new();
            reader.read_line(&mut request_line).await.unwrap();

            let redirect_url = request_line.split_whitespace().nth(1).unwrap();
            let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

            let code_pair = url
                .query_pairs()
                .find(|pair| {
                    let &(ref key, _) = pair;
                    key == "code"
                })
                .unwrap();

            let (_, value) = code_pair;
            code = AuthorizationCode::new(value.into_owned());

            let state_pair = url
                .query_pairs()
                .find(|pair| {
                    let &(ref key, _) = pair;
                    key == "state"
                })
                .unwrap();

            let (_, value) = state_pair;
            state = CsrfToken::new(value.into_owned());
        }

        let message = "Go back to your terminal :)";
        let response = format!(
            "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
            message.len(),
            message
        );
        stream.write_all(response.as_bytes()).await.unwrap();

        // Verify that the state we generated matches the one the server sent us.
        assert_eq!(
            csrf_state.secret(),
            state.secret(),
            "CSRF state mismatch. Malicious actor?"
        );

        // Exchange the code with a token.
        let token = match client
            .exchange_code(code)
            .request_async(async_http_client)
            .await
        {
            Ok(t) => t,
            Err(e) => {
                eprintln!("OAuth2: {}", e);
                eprintln!("Failed to exchange the code for a valid access_token.\nIncorrect client secret?");
                std::process::exit(1);
            }
        };

        return token;
    }

    unreachable!();
}

fn write_auth_token(token: &Token, path: &Path) -> Result<()> {
    std::fs::create_dir_all(path.parent().unwrap())?;
    let file = std::fs::File::create(path)?;
    serde_json::to_writer_pretty(file, &token)?;
    Ok(())
}

fn clear_auth_token(path: &Path) -> Result<()> {
    std::fs::remove_file(path)?;
    Ok(())
}

fn read_auth_token(path: &Path) -> Result<Token> {
    let file = std::fs::File::open(path)?;
    let token = serde_json::from_reader(file)?;
    Ok(token)
}
