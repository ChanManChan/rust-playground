pub mod register;

pub use register::Register;
pub use route::*;

pub mod route {
    pub const ACCOUNT_REGISTER: &'static str = "/account/register";
}
