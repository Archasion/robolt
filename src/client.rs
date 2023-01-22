use tokio_test::{assert_ok, assert_err};
use reqwest::{Client, Method};
use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;
use serde::Deserialize;

#[derive(Deserialize)]
struct RobloxResponseError {
    message: String
}

#[derive(Deserialize)]
struct RobloxResponseErrors {
    errors: Vec<RobloxResponseError>,
}

pub(crate) struct HttpClient {
    client: Client,
}

impl Default for HttpClient {
    fn default() -> Self {
        let user_agent = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

        Self {
            client: Client::builder()
                .user_agent(user_agent)
                .cookie_store(true)
                .build()
                .expect("Failed to build reqwest client"),
        }
    }
}

impl HttpClient {
    pub(crate) fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub(crate) async fn req<T>(&self, method: Method, url: &str, headers: Option<HeaderMap>) -> Result<T, String>
        where T: DeserializeOwned
    {
        let res = self.client.request(method, url)
            .headers(headers.unwrap_or_default())
            .send()
            .await;

        match res {
            Ok(res) => {
                if res.status().is_success() {
                    let body = res.json::<T>().await;
                    match body {
                        Ok(body) => Ok(body),
                        Err(_) => Err("Failed to deserialize response body".to_string()),
                    }
                } else {
                    let body = res.json::<RobloxResponseErrors>().await;
                    match body {
                        Ok(body) => {
                            let errors = body.errors;
                            let error = errors.first().unwrap();
                            Err(error.message.to_string())
                        }
                        Err(_) => Err("Failed to deserialize response body".to_string()),
                    }
                }
            },
            Err(_) => Err("Failed to send request".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn ok_get_req() {
        let client = HttpClient::new();

        let res = client.req::<serde_json::Value>(
            Method::GET,
            "https://httpbin.org/get",
            None
        ).await;

        assert_ok!(res);
    }

    #[tokio::test]
    async fn ok_post_req() {
        let client = HttpClient::new();
        let mut headers = HeaderMap::new();

        headers.insert("Content-Type", "application/json".parse().unwrap());

        let res = client.req::<serde_json::Value>(
            Method::POST,
            "https://httpbin.org/post",
            Some(headers)
        ).await;

        assert_ok!(res);
    }

    #[tokio::test]
    async fn err_get_req() {
        let client = HttpClient::new();
        let res = client.req::<serde_json::Value>(
            Method::GET,
            "https://httpbin.org/status/404",
            None
        ).await;

        assert_err!(res);
    }

    #[tokio::test]
    async fn err_post_req() {
        let client = HttpClient::new();
        let mut headers = HeaderMap::new();

        headers.insert("Content-Type", "application/json".parse().unwrap());

        let res = client.req::<serde_json::Value>(
            Method::POST,
            "https://httpbin.org/status/404",
            Some(headers)
        ).await;

        assert_err!(res);
    }

    #[tokio::test]
    async fn roblox_err_res() {
        let client = HttpClient::new();

        let res = client.req::<String>(
            Method::GET,
            "https://users.roblox.com/v1/users/0",
            None
        ).await;

        assert_err!(&res);
        assert_eq!(res.unwrap_err(), "The user id is invalid.");
    }
}