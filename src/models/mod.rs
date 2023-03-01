use serde::Deserialize;

mod auth;
pub(crate) mod badges;
pub(crate) mod friends;
mod points;
pub(crate) mod presence;
mod users;
mod economy;

struct RobloxAPIEndpoints<T> {
    presence: T,
    economy: T,
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
    economy: "economy.roblox.com",
    friends: "friends.roblox.com",
    points: "points.roblox.com",
    badges: "badges.roblox.com",
    users: "users.roblox.com",
    base: "api.roblox.com",
    web: "www.roblox.com",
};