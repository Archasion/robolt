pub mod users;

struct RobloxAPIEndpoints<T> {
    users: T,
}

const ENDPOINTS: RobloxAPIEndpoints<&'static str> = RobloxAPIEndpoints {
    users: "users.roblox.com",
};