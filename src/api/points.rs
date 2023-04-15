use serde::Deserialize;

use crate::api::routes::RobloxApi;
use crate::errors::RoboltError;
use crate::Robolt;

impl<State> Robolt<State> {
	pub async fn points(&self, user_id: u64, universe_id: u64) -> Result<u64, RoboltError> {
		self.request(
			RobloxApi::Points,
			format!("/v1/universes/{user_id}/users/{universe_id}/all-time"),
		)
		.send::<AllTimeScore>()
		.await
		.map(|res| res.all_time_score)
	}
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AllTimeScore {
	all_time_score: u64,
}