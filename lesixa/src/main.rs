fn main() {

}
struct user {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u32,
}
fn build_user() -> user {
    user {
        email,
        username,
        active: true,
        sign_in_count: 0,
    }
}