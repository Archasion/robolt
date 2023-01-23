pub mod users;
pub mod auth;

struct APIEndpoints<T> {
    users: T,
    base: T,
    web: T
}

const ENDPOINTS: APIEndpoints<&'static str> = APIEndpoints {
    users: "users.roblox.com",
    base: "api.roblox.com",
    web: "www.roblox.com"
};