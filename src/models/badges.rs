use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::models::{DataResponse, ENDPOINTS};
use crate::Robolt;

impl Robolt {
    pub fn fetch_badge(&self, id: u64) -> Result<Badge, String> {
        self.request_builder(format!("{}/v1/badges/{}", ENDPOINTS.badges, id))
            .send()
    }

    pub fn fetch_game_badges(&self, id: u64) -> Result<Vec<Badge>, String> {
        self.request_builder(format!(
            "{}/v1/universes/{}/badges?limit=100",
            ENDPOINTS.badges, id
        ))
            .send::<DataResponse<Badge>>()
            .map(|res| res.data)
    }

    pub fn fetch_user_badges(&self, id: u64) -> Result<Vec<Badge>, String> {
        self.request_builder(format!(
            "{}/v1/users/{}/badges?limit=100",
            ENDPOINTS.badges, id
        ))
            .send::<DataResponse<Badge>>()
            .map(|res| res.data)
    }

    pub fn fetch_user_awarded_badge_dates(
        &self,
        user_id: u64,
        badge_ids: Vec<u64>,
    ) -> Result<Vec<BadgeAwardDate>, String> {
        let badge_ids = badge_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        self.request_builder(format!(
            "{}/v1/users/{}/badges/awarded-dates?badgeIds={}",
            ENDPOINTS.badges, user_id, badge_ids
        ))
            .send::<DataResponse<BadgeAwardDate>>()
            .map(|res| res.data)
    }

    pub fn update_badge(&self, id: u64) -> BadgeUpdateBuilder {
        BadgeUpdateBuilder::new(id, self)
    }

    pub fn remove_badge(&self, id: u64) -> Result<(), String> {
        self.request_builder(format!("{}/v1/user/badges/{}", ENDPOINTS.badges, id))
            .method(Method::DELETE)
            .send()
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
        self.client
            .request_builder(format!("{}/v1/badges/{}", ENDPOINTS.badges, self.id))
            .method(Method::PATCH)
            .send_body(Some(self))
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
pub struct BadgeAwardDate {
    pub badge_id: u64,
    pub awarded_date: String,
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
    pub awarding_universe: Option<AwardingUniverse>,
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