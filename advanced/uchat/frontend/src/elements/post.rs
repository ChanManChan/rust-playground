use crate::prelude::*;
use dioxus::prelude::*;
use fermi::{use_atom_ref, UseAtomRef};
use indexmap::IndexMap;
use uchat_domain::ids::PostId;
use uchat_endpoint::post::types::PublicPost;

pub mod actionbar;
pub mod content;
pub mod header;
pub mod profile_image;
pub mod quick_respond;

pub fn use_post_manager(cx: &ScopeState) -> &UseAtomRef<PostManager> {
    use_atom_ref(cx, crate::app::POSTMANAGER)
}

#[derive(Default)]
pub struct PostManager {
    pub posts: IndexMap<PostId, PublicPost>,
}

impl PostManager {
    pub fn update<F>(&mut self, id: PostId, mut update_fn: F) -> bool
    where
        F: FnMut(&mut PublicPost),
    {
        if let Some(post) = self.posts.get_mut(&id) {
            update_fn(post);
            true
        } else {
            false
        }
    }
    pub fn populate<T>(&mut self, posts: T)
    where
        T: Iterator<Item = PublicPost>,
    {
        self.posts.clear();
        for post in posts {
            self.posts.insert(post.id, post);
        }
    }
    pub fn clear(&mut self) {
        self.posts.clear();
    }
    pub fn get(&self, post_id: &PostId) -> Option<&PublicPost> {
        self.posts.get(post_id)
    }
    pub fn remove(&mut self, post_id: &PostId) {
        self.posts.remove(post_id);
    }
    pub fn all_to_public<'a, 'b>(&self) -> Vec<LazyNodes<'a, 'b>> {
        self.posts
            .iter()
            .map(|(&id, _)| {
                rsx! {
                    div {
                        PublicPostEntry {
                            post_id: id
                        }
                    }
                }
            })
            .collect()
    }
}

#[inline_props]
pub fn PublicPostEntry(cx: Scope, post_id: PostId) -> Element {
    let post_manager = use_post_manager(cx);
    let _router = use_router(cx);

    let this_post = {
        let post = post_manager.read().get(post_id).unwrap().clone();
        use_state(cx, || post)
    };

    cx.render(rsx! {
        div {
            key: "{this_post.id.to_string()}",
            class: "grid grid-cols-[50px_1fr] gap-2 mb-4",
            profile_image::ProfileImage { post: this_post }
            div {
                class: "flex flex-col gap-3",
                header::Header { post: this_post },
                content::Content { post: this_post },
                actionbar::Actionbar { post_id: this_post.id },
                hr {}
            }
        }
    })
}
