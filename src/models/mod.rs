use serde::Deserialize;

pub mod users;

struct RobloxAPIEndpoints<T> {
    users: T,
    base: T,
}

#[derive(Debug, Deserialize)]
struct DataResponse<T> {
    data: T,
}

const ENDPOINTS: RobloxAPIEndpoints<&'static str> = RobloxAPIEndpoints {
    users: "users.roblox.com",
    base: "api.roblox.com",
};
