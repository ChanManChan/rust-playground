pub mod home;
pub mod login;
pub mod new_post;
pub mod register;

pub use home::Home;
pub use login::Login;
pub use new_post::*;
pub use register::Register;
pub use route::*;

pub mod route {
    pub const ACCOUNT_REGISTER: &'static str = "/account/register";
    pub const ACCOUNT_LOGIN: &'static str = "/account/login";
    pub const HOME: &'static str = "/home";
    pub const POST_NEW_CHAT: &'static str = "/post/new_chat";
}
