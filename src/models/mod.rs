use serde::Deserialize;

mod auth;
pub(crate) mod badges;
pub(crate) mod friends;
pub(crate) mod presence;
mod users;
mod points;

struct RobloxAPIEndpoints<T> {
    presence: T,
    friends: T,
    points: T,
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
    presence: "presence.roblox.com",
    friends: "friends.roblox.com",
    points: "points.roblox.com",
    badges: "badges.roblox.com",
    users: "users.roblox.com",
    base: "api.roblox.com",
    web: "www.roblox.com",
};