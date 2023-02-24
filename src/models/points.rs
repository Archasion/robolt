use serde::Deserialize;

use crate::models::ENDPOINTS;
use crate::Robolt;
use crate::utilities::client::Authenticated;

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

impl Robolt<Authenticated> {
    pub fn my_points(&self, universe_id: u64) -> Result<u64, String> {
        let user_id = self.fetch_current_user()?.id;
        self.fetch_points(user_id, universe_id)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AllTimeScore {
    all_time_score: u64,
}