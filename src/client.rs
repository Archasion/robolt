use reqwest::{Client, Method};
use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;

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
        let res = self.client.request(method, format!("https://{}", url))
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

    const ENDPOINT_GET: &str = "httpbin.org/get";
    const ENDPOINT_POST: &str = "httpbin.org/post";
    const ENDPOINT_404: &str = "httpbin.org/status/404";
    const ENDPOINT_ROBLOX: &str = "users.roblox.com/v1/users/0"; // Intentionally invalid user ID

    #[tokio::test]
    async fn ok_get_req() {
        let client = HttpClient::new();
        let res = client.req::<Value>(Method::GET, ENDPOINT_GET, None).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn ok_post_req() {
        let client = HttpClient::new();
        let mut headers = HeaderMap::new();

        headers.insert("Content-Type", "application/json".parse().unwrap());

        let res = client.req::<Value>(Method::POST, ENDPOINT_POST, Some(headers)).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn err_get_req() {
        let client = HttpClient::new();
        let res = client.req::<Value>(Method::GET, ENDPOINT_404, None).await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn err_post_req() {
        let client = HttpClient::new();
        let mut headers = HeaderMap::new();

        headers.insert("Content-Type", "application/json".parse().unwrap());

        let res = client.req::<Value>(Method::POST, ENDPOINT_404, Some(headers)).await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn roblox_err_res() {
        let client = HttpClient::new();
        let res = client.req::<String>(Method::GET, ENDPOINT_ROBLOX, None).await;

        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "The user id is invalid.");
    }
}