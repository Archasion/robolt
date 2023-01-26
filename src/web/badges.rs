use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::utilities::client::{HttpClientExt, HttpRequest, HTTP};
use crate::web::ENDPOINTS;

pub enum RobloxBadge {
    WelcomeToTheClub,
    Administrator,
    Veteran,
    Friendship,
    Ambassador,
    Inviter,
    Homestead,
    Bricksmith,
    OfficialModelMaker,
    CombatInitiation,
    Warrior,
    Bloxxer,
}

#[derive(Debug, Serialize)]
pub struct BadgeConfig {
    pub name: Option<&'static str>,
    #[serde(default)]
    pub description: Option<&'static str>,
    pub enabled: bool,
    #[serde(skip_serializing)]
    pub return_updated_badge: bool,
}

#[derive(Debug, Deserialize)]
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
    pub statistics: BadgeStatistics,
    pub awarding_universe: AwardingUniverse,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BadgeStatistics {
    pub past_day_awarded_count: u64,
    pub awarded_count: u64,
    pub win_rate_percentage: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AwardingUniverse {
    pub id: u64,
    pub name: String,
    pub root_place_id: u64,
}

pub fn fetch(id: u64) -> Result<Badge, String> {
    let req = HttpRequest {
        method: Method::GET,
        url: format!("{}/v1/badges/{}", ENDPOINTS.badges, id),
        headers: None,
        body: None,
        response: true,
    };

    HTTP.send::<Badge>(req).map(|badge| badge.unwrap())
}

pub fn remove(id: u64) -> Result<(), String> {
    let req = HttpRequest {
        method: Method::DELETE,
        url: format!("{}/v1/user/badges/{}", ENDPOINTS.badges, id),
        headers: None,
        body: None,
        response: false,
    };

    HTTP.send::<serde_json::Value>(req)?;
    Ok(())
}

pub fn update(id: u64, data: BadgeConfig) -> Result<Option<Badge>, String> {
    let config = serde_json::to_string(&data).expect("Failed to serialize badge configuration");

    let req = HttpRequest {
        method: Method::PATCH,
        url: format!("{}/v1/badges/{}", ENDPOINTS.badges, id),
        headers: None,
        body: Some(config),
        response: false,
    };

    HTTP.send::<serde_json::Value>(req)?;

    if data.return_updated_badge {
        return Ok(Some(fetch(id).unwrap()));
    }

    Ok(None)
}

impl Badge {
    pub fn remove(&self) -> Result<(), String> {
        remove(self.id)
    }

    pub fn update(&self, data: BadgeConfig) -> Result<Option<Badge>, String> {
        update(self.id, data)
    }
}