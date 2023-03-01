use std::collections::HashMap;

use reqwest::Method;
use serde::Deserialize;
use serde_repr::Deserialize_repr;

use crate::models::ENDPOINTS;
use crate::Robolt;

impl<State> Robolt<State> {
    pub fn fetch_presences(&self, user_ids: Vec<u64>) -> Result<Vec<Presence>, String> {
        let mut body = HashMap::new();
        body.insert("userIds", user_ids);

        self.request_builder(format!("{}/v1/presence/users", ENDPOINTS.presence))
            .method(Method::POST)
            .send_body::<_, Presences>(body)
            .map(|res| res.user_presences)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Presence {
    user_presence_type: PresenceType,
    last_location: String,
    last_online: String,
    user_id: u64,
    game_id: Option<u64>,
    place_id: Option<u64>,
    universe_id: Option<u64>,
    root_place_id: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize_repr)]
#[repr(u8)]
pub enum PresenceType {
    Offline = 0,
    Online = 1,
    InGame = 2,
    InStudio = 3,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Presences {
    user_presences: Vec<Presence>,
}