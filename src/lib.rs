use std::path::PathBuf;

use api::{body, body_time_series};
use oauth::Auth;
use reqwest::{StatusCode, Url};
use serde::{de::DeserializeOwned, Serialize};

mod api;
mod error;
mod oauth;
mod util;

pub mod models;

/// A convenience type with a default error type of [`Error`].
pub type Result<T, E = error::Error> = std::result::Result<T, E>;

const BASE_URL: &str = "https://api.fitbit.com";
pub struct Client {
    auth: Auth,
    client: reqwest::Client,
    base_url: Url,
}

impl Client {
    pub fn new(client_id: &str, client_secret: &str) -> Self {
        Self {
            base_url: Url::parse(BASE_URL).unwrap(),
            auth: Auth::new(client_id.to_owned(), client_secret.to_owned(), None),
            client: reqwest::ClientBuilder::new()
                .user_agent("fitbit-rs")
                .build()
                .unwrap(),
        }
    }
    pub fn with_cache<P>(mut self, path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        self.auth.with_cache(path);
        self
    }

    pub async fn auth_interactive(mut self) -> Self {
        self.auth.auth_interactive().await;
        self
    }

    pub fn body(&self) -> body::BodyHandler {
        body::BodyHandler::new(self)
    }
    pub fn body_time_series(&self) -> body_time_series::BodyTimeSeriesHandler {
        body_time_series::BodyTimeSeriesHandler::new(self)
    }
}

impl Client {
    pub fn absolute_url(&self, url: &str) -> Result<Url> {
        Ok(self.base_url.join(url)?)
    }
}

impl Client {
    /// Send a `POST` request to `route` with an optional body, returning the body
    /// of the response.
    pub async fn post<P: Serialize + ?Sized, R: DeserializeOwned>(
        &self,
        route: &str,
        body: Option<&P>,
    ) -> Result<R> {
        let response = self._post(self.absolute_url(route)?, body).await?;
        let text = response.text().await?;
        let json = serde_json::from_str(&text)?;
        Ok(json)
    }

    /// Send a `POST` request with no additional pre/post-processing.
    pub async fn _post<P: Serialize + ?Sized>(
        &self,
        url: impl reqwest::IntoUrl,
        body: Option<&P>,
    ) -> Result<reqwest::Response> {
        let mut request = self.client.post(url);

        if let Some(body) = body {
            request = request.json(body);
        }

        self.execute(request).await
    }

    /// Send a `GET` request to `route` with optional query parameters, returning
    /// the body of the response.
    pub async fn get<R, P>(&self, route: &str, parameters: Option<&P>) -> Result<R>
    where
        P: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let response = self._get(self.absolute_url(route)?, parameters).await?;
        let text = response.text().await?;
        let json = serde_json::from_str(&text)?;
        Ok(json)
    }

    /// Send a `GET` request with no additional post-processing.
    pub async fn _get<P: Serialize + ?Sized>(
        &self,
        url: impl reqwest::IntoUrl,
        parameters: Option<&P>,
    ) -> Result<reqwest::Response> {
        let mut request = self.client.get(url);

        if let Some(parameters) = parameters {
            request = request.query(parameters);
        }

        self.execute(request).await
    }

    /// Send a `PATCH` request to `route` with optional query parameters,
    /// returning the body of the response.
    pub async fn patch<R, B>(&self, route: &str, body: Option<&B>) -> Result<R>
    where
        B: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let response = self._patch(self.absolute_url(route)?, body).await?;
        let text = response.text().await?;
        let json = serde_json::from_str(&text)?;
        Ok(json)
    }

    /// Send a `PATCH` request with no additional post-processing.
    pub async fn _patch<B: Serialize + ?Sized>(
        &self,
        url: impl reqwest::IntoUrl,
        parameters: Option<&B>,
    ) -> Result<reqwest::Response> {
        let mut request = self.client.patch(url);

        if let Some(parameters) = parameters {
            request = request.json(parameters);
        }

        self.execute(request).await
    }

    /// Send a `PUT` request to `route` with optional query parameters,
    /// returning the body of the response.
    pub async fn put<R, B>(&self, route: &str, body: Option<&B>) -> Result<R>
    where
        B: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let response = self._put(self.absolute_url(route)?, body).await?;
        let text = response.text().await?;
        let json = serde_json::from_str(&text)?;
        Ok(json)
    }

    /// Send a `PATCH` request with no additional post-processing.
    pub async fn _put<B: Serialize + ?Sized>(
        &self,
        url: impl reqwest::IntoUrl,
        body: Option<&B>,
    ) -> Result<reqwest::Response> {
        let mut request = self.client.put(url);

        if let Some(body) = body {
            request = request.json(body);
        }

        self.execute(request).await
    }

    /// Send a `DELETE` request to `route` with optional query parameters,
    /// returning the body of the response.
    pub async fn delete<R, A, P>(&self, route: &str, parameters: Option<&P>) -> Result<R>
    where
        P: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let response = self._delete(self.absolute_url(route)?, parameters).await?;
        let text = response.text().await?;
        let json = serde_json::from_str(&text)?;
        Ok(json)
    }

    /// Send a `DELETE` request with no additional post-processing.
    pub async fn _delete<P: Serialize + ?Sized>(
        &self,
        url: impl reqwest::IntoUrl,
        parameters: Option<&P>,
    ) -> Result<reqwest::Response> {
        let mut request = self.client.delete(url);

        if let Some(parameters) = parameters {
            request = request.query(parameters);
        }

        self.execute(request).await
    }

    /// Execute the given `request` using the Client.
    pub async fn execute(&self, mut request: reqwest::RequestBuilder) -> Result<reqwest::Response> {
        let token = match self.auth.get_token() {
            Some(t) => t,
            None => {
                eprintln!("Couldn't get token. Ensure you've called interactive_auth() first.");
                std::process::exit(1);
            }
        };
        request = request.bearer_auth(token);

        let result = request.send().await?;
        let status = result.status();
        if !status.is_success() {
            eprintln!("Bad HTTP status code: {}", status);
            match status {
                StatusCode::UNAUTHORIZED => {
                    eprintln!("Check that your API token is correct?");
                    eprintln!("Error: {}", result.text().await.unwrap())
                }
                StatusCode::BAD_REQUEST => {
                    eprintln!("Error: {}", result.text().await.unwrap())
                }
                _ => (),
            };
            std::process::exit(1);
        };
        Ok(result)
    }
}
