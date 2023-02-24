use std::str::FromStr;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::models::{DataResponse, ENDPOINTS};
use crate::Robolt;
use crate::utilities::client::Authenticated;

impl<State> Robolt<State> {
    pub fn fetch_badge(&self, badge_id: u64) -> Result<Badge, String> {
        self.request_builder(format!("{}/v1/badges/{}", ENDPOINTS.badges, badge_id))
            .send()
    }

    pub fn fetch_game_badges(&self, universe_id: u64) -> Result<Vec<Badge>, String> {
        self.request_builder(format!(
            "{}/v1/universes/{}/badges?limit=100",
            ENDPOINTS.badges, universe_id
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

    pub fn fetch_awarded_timestamps(
        &self,
        user_id: u64,
        badge_ids: Vec<u64>,
    ) -> Result<Vec<AwardedBadgeTimestamp>, String> {
        let badge_ids = badge_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        self.request_builder(format!(
            "{}/v1/users/{}/badges/awarded-dates?badgeIds={}",
            ENDPOINTS.badges, user_id, badge_ids
        ))
            .send::<DataResponse<AwardedBadgeTimestamp>>()
            .map(|res| res.data)
    }

    pub fn has_badge(&self, user_id: u64, badge_id: u64) -> Result<bool, String> {
        self.fetch_awarded_timestamps(user_id, vec![badge_id])
            .map(|badges| !badges.is_empty())
    }

    pub fn has_badges(&self, user_id: u64, badge_ids: Vec<u64>) -> Result<bool, String> {
        let badge_ids_len = badge_ids.len();

        self.fetch_awarded_timestamps(user_id, badge_ids)
            .map(|badges| badges.len() == badge_ids_len)
    }

    pub fn has_badges_any(&self, user_id: u64, badge_ids: Vec<u64>) -> Result<bool, String> {
        self.fetch_awarded_timestamps(user_id, badge_ids)
            .map(|badges| !badges.is_empty())
    }

    pub fn fetch_roblox_badges(&self, user_id: u64) -> Result<Vec<RobloxBadge>, String> {
        self.request_builder(format!(
            "{}/badges/roblox?userId={}",
            ENDPOINTS.web, user_id
        ))
            .send::<RobloxBadgesResult>()
            .map(|res| {
                res.roblox_badges
                    .into_iter()
                    .map(|badge| RobloxBadge::from_str(&badge.name).unwrap())
                    .collect()
            })
    }

    pub fn has_roblox_badge(&self, user_id: u64, badge: RobloxBadge) -> Result<bool, String> {
        self.fetch_roblox_badges(user_id)
            .map(|owned_badges| owned_badges.contains(&badge))
    }

    pub fn has_roblox_badges(
        &self,
        user_id: u64,
        badges: Vec<RobloxBadge>,
    ) -> Result<bool, String> {
        self.fetch_roblox_badges(user_id)
            .map(|owned_badges| badges.iter().all(|badge| owned_badges.contains(badge)))
    }

    pub fn has_roblox_badges_any(
        &self,
        user_id: u64,
        badges: Vec<RobloxBadge>,
    ) -> Result<bool, String> {
        self.fetch_roblox_badges(user_id)
            .map(|owned_badges| badges.iter().any(|badge| owned_badges.contains(badge)))
    }
}

impl Robolt<Authenticated> {
    pub fn update_badge(&self, badge_id: u64) -> BadgeUpdateBuilder {
        BadgeUpdateBuilder::new(badge_id, self)
    }

    pub fn remove_badge(&self, badge_id: u64) -> Result<(), String> {
        self.request_builder(format!("{}/v1/user/badges/{}", ENDPOINTS.badges, badge_id))
            .method(Method::DELETE)
            .send()
    }
}

impl<'a> BadgeUpdateBuilder<'a> {
    fn new(id: u64, client: &'a Robolt<Authenticated>) -> Self {
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

impl FromStr for RobloxBadge {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Welcome To The Club" => Ok(RobloxBadge::WelcomeToTheClub),
            "Administrator" => Ok(RobloxBadge::Administrator),
            "Veteran" => Ok(RobloxBadge::Veteran),
            "Friendship" => Ok(RobloxBadge::Friendship),
            "Ambassador" => Ok(RobloxBadge::Ambassador),
            "Inviter" => Ok(RobloxBadge::Inviter),
            "Homestead" => Ok(RobloxBadge::Homestead),
            "Bricksmith" => Ok(RobloxBadge::Bricksmith),
            "Official Model Maker" => Ok(RobloxBadge::OfficialModelMaker),
            "Combat Initiation" => Ok(RobloxBadge::CombatInitiation),
            "Warrior" => Ok(RobloxBadge::Warrior),
            "Bloxxer" => Ok(RobloxBadge::Bloxxer),
            _ => Err(format!("Unknown badge: {s}")),
        }
    }
}

#[derive(Serialize)]
pub struct BadgeUpdateBuilder<'a> {
    #[serde(skip_serializing)]
    id: u64,
    #[serde(skip_serializing)]
    client: &'a Robolt<Authenticated>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AwardedBadgeTimestamp {
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
    pub statistics: BadgeStats,
    pub awarding_universe: Option<AwardingUniverse>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BadgeStats {
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

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct RobloxBadgesResult {
    roblox_badges: Vec<RobloxBadgeResult>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct RobloxBadgeResult {
    name: String,
}

#[derive(Debug, Clone, PartialEq)]
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