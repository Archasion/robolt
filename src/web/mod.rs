pub mod users;

struct APIEndpoints<T> {
    users: T,
}

const ENDPOINTS: APIEndpoints<&'static str> = APIEndpoints {
    users: "users.roblox.com",
};