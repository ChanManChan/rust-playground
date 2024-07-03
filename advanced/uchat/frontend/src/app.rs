#![allow(non_snake_case)]

use crate::elements::toaster::ToastRoot;
use crate::elements::{sidebar::Sidebar, toaster::Toaster, Navbar};
use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use fermi::{use_init_atom_root, AtomRef};

pub static TOASTER: AtomRef<Toaster> = |_| Toaster::default();
pub static POSTMANAGER: AtomRef<PostManager> = |_| PostManager::default();
pub static LOCAL_PROFILE: AtomRef<LocalProfile> = |_| LocalProfile::default();
pub static SIDEBAR: AtomRef<SidebarManager> = |_| SidebarManager::default();

pub fn Init(cx: Scope) -> Element {
    let api_client = ApiClient::global();
    let toaster = use_toaster(cx);
    let router = use_router(cx);
    let local_profile = use_local_profile(cx);

    let _fetch_local_profile = {
        to_owned![api_client, toaster, local_profile, router];
        use_future(cx, (), |_| async move {
            use uchat_endpoint::user::endpoint::{GetMyProfile, GetMyProfileOk};
            let response = fetch_json!(<GetMyProfileOk>, api_client, GetMyProfile);
            match response {
                Ok(res) => {
                    local_profile.write().image = res.profile_image;
                    local_profile.write().user_id = Some(res.user_id);
                }
                Err(_e) => {
                    toaster.write().error(
                        "Please log in or create an account to continue",
                        chrono::Duration::seconds(3),
                    );
                    router.navigate_to(page::ACCOUNT_LOGIN)
                }
            }
        })
    };

    None
}

pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);
    let toaster = use_toaster(cx);
    cx.render(rsx! {
        Router {
            Init {}
            Sidebar {}
            main {
                class: "max-w-[var(--content-max-width)] min-w-[var(--content-min-width)] mb-[var(--navbar-height)] mt-[var(--appbar-height)] mx-auto p-4",
                Route { to: page::ACCOUNT_REGISTER, page::Register {} }
                Route { to: page::ACCOUNT_LOGIN, page::Login {} }
                Route { to: page::HOME, page::Home {} }
                Route { to: page::HOME_BOOKMARKED, page::HomeBookmarked {} }
                Route { to: page::HOME_LIKED, page::HomeLiked {} }
                Route { to: page::POST_NEW_CHAT, page::NewChat {} }
                Route { to: page::POST_NEW_IMAGE, page::NewImage {} }
                Route { to: page::POST_NEW_POLL, page::NewPoll {} }
                Route { to: page::POST_TRENDING, page::Trending {} }
                Route { to: page::PROFILE_EDIT, page::EditProfile {} }
                Route { to: page::PROFILE_VIEW, page::ViewProfile {} }
            }
            ToastRoot { toaster: toaster }
            Navbar {}
        }
    })
}
