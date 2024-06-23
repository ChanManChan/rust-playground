pub mod login;
pub mod register;

pub use login::Login;
pub use register::Register;
pub use route::*;

pub mod route {
    pub const ACCOUNT_REGISTER: &'static str = "/account/register";
    pub const ACCOUNT_LOGIN: &'static str = "/account/login";
}
