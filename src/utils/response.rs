use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct DataResponse<T> {
	pub(crate) data: Vec<T>,
}

#[derive(Deserialize)]
pub(crate) struct CountResponse<T> {
	pub(crate) count: T,
}

#[derive(Deserialize)]
pub(crate) struct EmptyResponse {}
