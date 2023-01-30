pub mod users;

struct RobloxAPIEndpoints<T> {
    users: T,
    base: T,
}

const ENDPOINTS: RobloxAPIEndpoints<&'static str> = RobloxAPIEndpoints {
    users: "users.roblox.com",
    base: "api.roblox.com",
};
