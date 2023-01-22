#![allow(unused)]

use serde::de::DeserializeOwned;
use reqwest::header::HeaderMap;
use reqwest::blocking::Client;
use reqwest::Method;

use crate::errors::RobloxAPIResponseErrors;

pub(crate) struct HttpClient {
    client: Client,
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpClient {
    pub(crate) fn new() -> Self {
        let user_agent = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

        Self {
            client: Client::builder()
                .user_agent(user_agent)
                .cookie_store(true)
                .build()
                .expect("Failed to build reqwest client"),
        }
    }

    pub(crate) fn req<T>(&self, method: Method, url: &str, headers: Option<HeaderMap>) -> Result<T, String>
        where T: DeserializeOwned
    {
        let res = self.client.request(method, format!("https://{url}"))
            .headers(headers.unwrap_or_default())
            .send();

        match res {
            Ok(res) => {
                let status_code = res.status();

                if res.status().is_success() {
                    let body = res.json::<T>();
                    match body {
                        Ok(body) => Ok(body),
                        Err(_) => Err("Failed to deserialize response body".to_string()),
                    }
                } else {
                    let body = res.json::<RobloxAPIResponseErrors>();
                    match body {
                        Ok(body) => {
                            let errors = body.errors;
                            let error = errors.first().unwrap();
                            Err(error.message.to_string())
                        }
                        Err(_) => Err(status_code.to_string())
                    }
                }
            },
            Err(_) => Err("Failed to send request".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use super::*;

    const ENDPOINT_GET: &str = "httpbin.org/get";
    const ENDPOINT_POST: &str = "httpbin.org/post";
    const ENDPOINT_404: &str = "httpbin.org/status/404";
    const ENDPOINT_ROBLOX: &str = "users.roblox.com/v1/users/0"; // Intentionally invalid user ID

    #[test]
    fn ok_get_req() {
        let client = HttpClient::new();
        let res = client.req::<Value>(Method::GET, ENDPOINT_GET, None);
        assert!(res.is_ok());
    }

    #[test]
    fn ok_post_req() {
        let client = HttpClient::new();
        let mut headers = HeaderMap::new();

        headers.insert("Content-Type", "application/json".parse().unwrap());

        let res = client.req::<Value>(Method::POST, ENDPOINT_POST, Some(headers));
        assert!(res.is_ok());
    }

    #[test]
    fn err_get_req() {
        let client = HttpClient::new();
        let res = client.req::<Value>(Method::GET, ENDPOINT_404, None);
        assert_eq!(res.unwrap_err(), "404 Not Found");
    }

    #[test]
    fn err_post_req() {
        let client = HttpClient::new();
        let mut headers = HeaderMap::new();

        headers.insert("Content-Type", "application/json".parse().unwrap());

        let res = client.req::<Value>(Method::POST, ENDPOINT_404, Some(headers));
        assert_eq!(res.unwrap_err(), "404 Not Found");
    }

    #[test]
    fn roblox_err_res() {
        let client = HttpClient::new();
        let res = client.req::<String>(Method::GET, ENDPOINT_ROBLOX, None);

        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "The user id is invalid.");
    }
}