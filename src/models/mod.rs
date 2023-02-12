use serde::Deserialize;

mod auth;
mod badges;
mod users;

struct RobloxAPIEndpoints<T> {
    badges: T,
    users: T,
    base: T,
}

#[derive(Debug, Deserialize)]
struct DataResponse<T> {
    data: Vec<T>,
}

const ENDPOINTS: RobloxAPIEndpoints<&'static str> = RobloxAPIEndpoints {
    badges: "badges.roblox.com",
    users: "users.roblox.com",
    base: "api.roblox.com",
};
