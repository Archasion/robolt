use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::models::ENDPOINTS;
use crate::Robolt;
use crate::utilities::client::HttpRequest;

impl Robolt {
    pub fn fetch_badge(&self, id: u64) -> Result<Badge, String> {
        let req = HttpRequest {
            method: Method::GET,
            endpoint: format!("{}/v1/badges/{}", ENDPOINTS.badges, id),
            body: None,
        };

        self.request::<(), Badge>(req)
    }

    pub fn update_badge(&self, id: u64) -> BadgeUpdateBuilder {
        BadgeUpdateBuilder::new(id, self)
    }
}

impl<'a> BadgeUpdateBuilder<'a> {
    fn new(id: u64, client: &'a Robolt) -> Self {
        Self {
            id,
            client,
            name: None,
            description: None,
            enabled: None,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }

    pub fn update(self) -> Result<(), String> {
        let req = HttpRequest {
            method: Method::PATCH,
            endpoint: format!("{}/v1/badges/{}", ENDPOINTS.badges, self.id),
            body: Some(&self),
        };

        self.client.request::<_, serde_json::Value>(req).map(|_| ())
    }
}

#[derive(Serialize)]
pub struct BadgeUpdateBuilder<'a> {
    #[serde(skip_serializing)]
    id: u64,
    #[serde(skip_serializing)]
    client: &'a Robolt,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Badge {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub display_name: String,
    pub display_description: Option<String>,
    pub enabled: bool,
    pub icon_image_id: u64,
    pub display_icon_image_id: u64,
    pub created: String,
    pub updated: String,
    pub statistics: BadgeAwardStatistics,
    pub awarding_universe: AwardingUniverse,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BadgeAwardStatistics {
    pub past_day_awarded_count: u64,
    pub awarded_count: u64,
    pub win_rate_percentage: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AwardingUniverse {
    pub id: u64,
    pub name: String,
    pub root_place_id: u64,
}