use serde::{Deserialize, Serialize};

pub mod post;
pub mod user;

pub trait Endpoint {
    const URL: &'static str;
    fn url(&self) -> &'static str {
        Self::URL
    }
}

macro_rules! route {
    ($url: literal => $request_type: ty) => {
        impl Endpoint for $request_type {
            const URL: &'static str = $url;
        }
    };
}

#[derive(thiserror::Error, Debug, Deserialize, Serialize)]
#[error("{msg}")]
pub struct RequestFailed {
    pub msg: String,
}

// public routes
route!("/account/login" => user::endpoint::Login);
route!("/account/create" => user::endpoint::CreateUser);

// authorized routes
route!("/post/new" => post::endpoint::NewPost);
route!("/posts/trending" => post::endpoint::TrendingPosts);
route!("/post/bookmark" => post::endpoint::Bookmark);
route!("/post/react" => post::endpoint::React);
