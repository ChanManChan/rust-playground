use dioxus::prelude::*;

use dioxus::core_macro::inline_props;
use dioxus_router::{use_router, RouterContext};
use uchat_domain::ids::UserId;
use uchat_endpoint::post::types::PublicPost;

use crate::sync_handler;

pub fn view_profile_onclick(
    router: &RouterContext,
    user_id: UserId,
) -> impl FnMut(MouseEvent) + '_ {
    sync_handler!([router], move |_| {
        let route = crate::page::route::profile_view(user_id);
        router.navigate_to(&route)
    })
}

#[inline_props]
pub fn ProfileImage<'a>(cx: Scope<'a>, post: &'a PublicPost) -> Element {
    let router = use_router(cx);

    let poster_info = &post.by_user;
    let profile_img_src = poster_info
        .profile_image
        .as_ref()
        .map(|url| url.as_str())
        .unwrap_or_else(|| "");

    cx.render(rsx! {
        div {
            img {
                class: "profile-portrait cursor-pointer",
                onclick: view_profile_onclick(router, poster_info.id),
                src: "{profile_img_src}"
            }
        }
    })
}
