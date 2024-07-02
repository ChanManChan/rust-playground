pub mod edit_profile;
pub mod home;
pub mod login;
pub mod new_post;
pub mod register;
pub mod trending;
pub mod view_profile;

pub use edit_profile::EditProfile;
pub use home::bookmarked::HomeBookmarked;
pub use home::liked::HomeLiked;
pub use home::Home;
pub use login::Login;
pub use new_post::*;
pub use register::Register;
pub use route::*;
pub use trending::Trending;
pub use view_profile::ViewProfile;

pub mod route {
    use uchat_domain::ids::UserId;

    pub const ACCOUNT_REGISTER: &'static str = "/account/register";
    pub const ACCOUNT_LOGIN: &'static str = "/account/login";
    pub const HOME: &'static str = "/home";
    pub const HOME_BOOKMARKED: &'static str = "/home/bookmarked";
    pub const HOME_LIKED: &'static str = "/home/liked";
    pub const POST_NEW_CHAT: &'static str = "/post/new_chat";
    pub const POST_NEW_IMAGE: &'static str = "/post/new_image";
    pub const POST_NEW_POLL: &'static str = "/post/new_poll";
    pub const POST_TRENDING: &'static str = "/posts/trending";
    pub const PROFILE_EDIT: &'static str = "/profile/edit";
    pub const PROFILE_VIEW: &'static str = "/profile/view/:user";

    pub fn profile_view(user_id: UserId) -> String {
        PROFILE_VIEW.replace(":user", &user_id.to_string())
    }
}
