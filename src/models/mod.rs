use serde::Deserialize;

mod auth;
pub(crate) mod badges;
pub(crate) mod friends;
mod users;

struct RobloxAPIEndpoints<T> {
    friends: T,
    badges: T,
    users: T,
    base: T,
    web: T,
}

#[derive(Debug, Deserialize)]
struct DataResponse<T> {
    data: Vec<T>,
}

const ENDPOINTS: RobloxAPIEndpoints<&'static str> = RobloxAPIEndpoints {
    friends: "friends.roblox.com",
    badges: "badges.roblox.com",
    users: "users.roblox.com",
    base: "api.roblox.com",
    web: "www.roblox.com",
};
