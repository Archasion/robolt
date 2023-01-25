pub mod auth;
pub mod badges;
pub mod users;

struct APIEndpoints<T> {
    badges: T,
    users: T,
    base: T,
    web: T,
}

const ENDPOINTS: APIEndpoints<&'static str> = APIEndpoints {
    badges: "badges.roblox.com",
    users: "users.roblox.com",
    base: "api.roblox.com",
    web: "www.roblox.com",
};
