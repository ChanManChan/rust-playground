#![allow(non_snake_case)]

use crate::prelude::*;
use dioxus::prelude::*;

pub fn HomeBookmarked(cx: Scope) -> Element {
    let toaster = use_toaster(cx);
    let api_client = ApiClient::global();
    let post_manager = use_post_manager(cx);
    let router = use_router(cx);

    let _fetch_posts = {
        to_owned![api_client, toaster, post_manager];
        use_future(cx, (), |_| async move {
            use uchat_endpoint::post::endpoint::{BookmarkedPosts, BookmarkedPostsOk};
            toaster
                .write()
                .info("Retrieving posts...", chrono::Duration::seconds(3));
            let response = fetch_json!(<BookmarkedPostsOk>, api_client, BookmarkedPosts);
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
        Appbar {
            title: "Bookmarked",
            AppbarImgButton {
                click_handler: move |_| router.navigate_to(page::HOME_LIKED),
                img: "/static/icons/icon-like.svg",
                label: "Liked",
                title: "Show liked posts",
            },
            AppbarImgButton {
                click_handler: move |_| (),
                img: "/static/icons/icon-bookmark.svg",
                label: "Saved",
                title: "Show bookmarked posts",
                disabled: true,
                append_class: appbar::BUTTON_SELECTED,
            },
            AppbarImgButton {
                click_handler: move |_| router.navigate_to(page::HOME),
                img: "/static/icons/icon-home.svg",
                label: "Home",
                title: "Go to the home page",
            },
        },
        Posts.into_iter()
    })
}