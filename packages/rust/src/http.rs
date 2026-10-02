//! HTTP client for NOWPayments API

use crate::error::NowPaymentsError;
use reqwest::{Client, Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::time::Duration;

/// HTTP client wrapper for NOWPayments API
pub struct HttpClient {
    client: Client,
    base_url: String,
}

fn api_error_message(data: &Option<Value>, text: &str) -> String {
    data.as_ref()
        .and_then(|d| d.get("message").and_then(|m| m.as_str()))
        .or_else(|| data.as_ref().and_then(|d| d.get("msg").and_then(|m| m.as_str())))
        .or_else(|| data.as_ref().and_then(|d| d.get("error").and_then(|e| e.as_str())))
        .unwrap_or(text)
        .to_string()
}

impl HttpClient {
    fn map_send_err(e: reqwest::Error) -> NowPaymentsError {
        if e.is_timeout() {
            NowPaymentsError::new(
                "Request timed out. Check your connection or try again.",
                None,
                None,
                None,
            )
        } else {
            NowPaymentsError::new(e.to_string(), None, None, None)
        }
    }

    pub fn new(base_url: &str, api_key: &str, timeout: Duration) -> Result<Self, NowPaymentsError> {
        let client = Client::builder()
            .timeout(timeout)
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::CONTENT_TYPE,
                    "application/json".parse().unwrap(),
                );
                headers.insert(
                    reqwest::header::HeaderName::from_static("x-api-key"),
                    api_key
                        .parse()
                        .map_err(|_| NowPaymentsError::new("Invalid API key format", None, None, None))?,
                );
                headers
            })
            .build()
            .map_err(|e| NowPaymentsError::new(e.to_string(), None, None, None))?;
        Ok(Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
        })
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    fn build_request(&self, method: Method, path: &str) -> RequestBuilder {
        self.client.request(method, self.url(path))
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, NowPaymentsError> {
        self.get_with_params(path, &[]).await
    }

    pub async fn get_with_params<T: DeserializeOwned>(
        &self,
        path: &str,
        params: &[(&str, String)],
    ) -> Result<T, NowPaymentsError> {
        let mut req = self.build_request(Method::GET, path);
        if !params.is_empty() {
            let query: Vec<_> = params
                .iter()
                .map(|(k, v)| (*k, v.as_str()))
                .collect();
            req = req.query(&query);
        }
        let resp = req.send().await.map_err(Self::map_send_err)?;
        Self::handle_response(resp).await
    }

    pub async fn get_raw(&self, path: &str) -> Result<Value, NowPaymentsError> {
        self.get_with_params(path, &[] as &[(&str, String)]).await
    }

    pub async fn post<T: DeserializeOwned>(&self, path: &str, body: Value) -> Result<T, NowPaymentsError> {
        self.post_with_headers(path, body, &[]).await
    }

    /// POST with optional extra headers (e.g. origin-ip for fiat2crypto).
    pub async fn post_with_headers<T: DeserializeOwned>(
        &self,
        path: &str,
        body: Value,
        headers: &[(&str, &str)],
    ) -> Result<T, NowPaymentsError> {
        let mut req = self.build_request(Method::POST, path).json(&body);
        for (k, v) in headers {
            req = req.header(*k, *v);
        }
        let resp = req.send().await.map_err(Self::map_send_err)?;
        Self::handle_response(resp).await
    }

    pub async fn delete<T: DeserializeOwned>(
        &self,
        path: &str,
        jwt_token: Option<&str>,
    ) -> Result<T, NowPaymentsError> {
        let mut req = self.build_request(Method::DELETE, path);
        if let Some(token) = jwt_token {
            req = req.bearer_auth(token);
        }
        let resp = req.send().await.map_err(Self::map_send_err)?;
        Self::handle_response(resp).await
    }

    pub async fn patch<T: DeserializeOwned>(
        &self,
        path: &str,
        body: Value,
    ) -> Result<T, NowPaymentsError> {
        let resp = self
            .build_request(Method::PATCH, path)
            .json(&body)
            .send()
            .await
            .map_err(Self::map_send_err)?;
        Self::handle_response(resp).await
    }

    pub async fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<Value>,
    ) -> Result<T, NowPaymentsError> {
        let mut req = self.build_request(method, path);
        if let Some(b) = body {
            req = req.json(&b);
        }
        let resp = req.send().await.map_err(Self::map_send_err)?;
        Self::handle_response(resp).await
    }

    pub async fn post_with_auth<T: DeserializeOwned>(
        &self,
        path: &str,
        body: Value,
        jwt_token: &str,
    ) -> Result<T, NowPaymentsError> {
        let resp = self
            .build_request(Method::POST, path)
            .bearer_auth(jwt_token)
            .json(&body)
            .send()
            .await
            .map_err(Self::map_send_err)?;
        Self::handle_response(resp).await
    }

    pub async fn get_with_auth<T: DeserializeOwned>(
        &self,
        path: &str,
        params: &[(&str, String)],
        jwt_token: Option<&str>,
    ) -> Result<T, NowPaymentsError> {
        let mut req = self.build_request(Method::GET, path);
        if !params.is_empty() {
            let query: Vec<_> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            req = req.query(&query);
        }
        if let Some(token) = jwt_token {
            req = req.bearer_auth(token);
        }
        let resp = req.send().await.map_err(Self::map_send_err)?;
        Self::handle_response(resp).await
    }

    /// Alias for get_with_auth
    pub async fn get_with_params_auth<T: DeserializeOwned>(
        &self,
        path: &str,
        params: &[(&str, String)],
        jwt_token: Option<&str>,
    ) -> Result<T, NowPaymentsError> {
        self.get_with_auth(path, params, jwt_token).await
    }

    /// GET request with optional JWT auth, no query params
    pub async fn get_auth<T: DeserializeOwned>(
        &self,
        path: &str,
        jwt_token: Option<&str>,
    ) -> Result<T, NowPaymentsError> {
        self.get_with_auth(path, &[], jwt_token).await
    }

    /// Alias for post_with_auth
    pub async fn post_auth<T: DeserializeOwned>(
        &self,
        path: &str,
        body: Value,
        jwt_token: &str,
    ) -> Result<T, NowPaymentsError> {
        self.post_with_auth(path, body, jwt_token).await
    }

    /// POST with JWT when the API returns no response body (e.g. cancel payout).
    pub async fn post_auth_no_content(
        &self,
        path: &str,
        body: Value,
        jwt_token: &str,
    ) -> Result<(), NowPaymentsError> {
        self.post_auth_text(path, body, jwt_token).await.map(|_| ())
    }

    /// POST with JWT returning raw response text (e.g. verify payout returns plain "OK").
    pub async fn post_auth_text(
        &self,
        path: &str,
        body: Value,
        jwt_token: &str,
    ) -> Result<String, NowPaymentsError> {
        let resp = self
            .build_request(Method::POST, path)
            .bearer_auth(jwt_token)
            .json(&body)
            .send()
            .await
            .map_err(Self::map_send_err)?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if status.is_success() {
            return Ok(text.trim().trim_matches('"').to_string());
        }
        let data: Option<serde_json::Value> = serde_json::from_str(&text).ok();
        let message = api_error_message(&data, &text);
        let code = data
            .as_ref()
            .and_then(|d| d.get("code"))
            .and_then(|c| c.as_str())
            .map(String::from);
        Err(NowPaymentsError::new(
            message,
            Some(status.as_u16()),
            code,
            data,
        ))
    }

    async fn handle_response<T: DeserializeOwned>(
        resp: reqwest::Response,
    ) -> Result<T, NowPaymentsError> {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();

        if !status.is_client_error() && !status.is_server_error() {
            if text.trim().is_empty() {
                return serde_json::from_str("null").map_err(|e| {
                    NowPaymentsError::new(
                        &format!("Failed to parse empty response: {}", e),
                        Some(status.as_u16()),
                        None,
                        None,
                    )
                });
            }
            serde_json::from_str(&text).map_err(|e| {
                NowPaymentsError::new(
                    &format!("Failed to parse response: {}", e),
                    Some(status.as_u16()),
                    None,
                    Some(serde_json::Value::String(text)),
                )
            })
        } else {
            let data: Option<serde_json::Value> = serde_json::from_str(&text).ok();
            let message = api_error_message(&data, &text);
            let code = data
                .as_ref()
                .and_then(|d| d.get("code"))
                .and_then(|c| c.as_str())
                .map(String::from);
            Err(NowPaymentsError::new(
                message,
                Some(status.as_u16()),
                code,
                data,
            ))
        }
    }
}
