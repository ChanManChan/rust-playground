#![allow(non_snake_case)]

use crate::elements::post::PostManager;
use crate::elements::toaster::ToastRoot;
use crate::elements::{toaster::Toaster, Navbar};
use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use fermi::{use_init_atom_root, AtomRef};

pub static TOASTER: AtomRef<Toaster> = |_| Toaster::default();
pub static POSTMANAGER: AtomRef<PostManager> = |_| PostManager::default();

pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);
    let toaster = use_toaster(cx);
    cx.render(rsx! {
        Router {
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
            }
            ToastRoot { toaster: toaster }
            Navbar {}
        }
    })
}
