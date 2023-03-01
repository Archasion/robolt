use serde::Deserialize;

use crate::models::ENDPOINTS;
use crate::Robolt;

impl<State> Robolt<State> {
    pub fn fetch_points(&self, user_id: u64, universe_id: u64) -> Result<u64, String> {
        self.request_builder(format!(
            "{}/v1/universes/{}/users/{}/all-time",
            ENDPOINTS.points, user_id, universe_id
        ))
            .send::<AllTimeScore>()
            .map(|res| res.all_time_score)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AllTimeScore {
    all_time_score: u64,
}