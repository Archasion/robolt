use reqwest::Method;
use serde::Deserialize;

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
    };

    HTTP.request::<Badge>(req)
}

pub fn update() {
    todo!();
}

pub fn remove() {
    todo!();
}