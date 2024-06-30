#![allow(non_snake_case)]

use crate::prelude::*;
use chrono::Duration;
use dioxus::prelude::*;

pub fn Home(cx: Scope) -> Element {
    let toaster = use_toaster(cx);
    let api_client = ApiClient::global();
    let post_manager = use_post_manager(cx);
    let router = use_router(cx);

    let _fetch_posts = {
        to_owned![api_client, toaster, post_manager];
        use_future(cx, (), |_| async move {
            use uchat_endpoint::post::endpoint::{HomePosts, HomePostsOk};
            toaster
                .write()
                .info("Retrieving posts...", chrono::Duration::seconds(3));
            let response = fetch_json!(<HomePostsOk>, api_client, HomePosts);
            match response {
                Ok(res) => post_manager.write().populate(res.posts.into_iter()),
                Err(e) => toaster.write().error(
                    format!("Failed to retrieve posts: {e}"),
                    chrono::Duration::seconds(3),
                ),
            }
        });
    };

    let Posts = post_manager.read().all_to_public();

    cx.render(rsx! {
        Posts.into_iter()
    })
}
