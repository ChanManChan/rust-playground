#![allow(non_snake_case)]

use crate::{
    elements::keyed_notification_box::{KeyedNotificationBox, KeyedNotifications},
    fetch_json,
    prelude::*,
    util::api_client::ApiClient,
};
use dioxus::prelude::*;
use dioxus_router::Link;
use uchat_domain::UserFacingError;

pub struct PageState {
    username: UseState<String>,
    password: UseState<String>,
    form_errors: KeyedNotifications,
    server_messages: KeyedNotifications,
}

impl PageState {
    pub fn new(cx: Scope) -> Self {
        Self {
            username: use_state(cx, String::new).clone(),
            password: use_state(cx, String::new).clone(),
            form_errors: KeyedNotifications::default(),
            server_messages: KeyedNotifications::default(),
        }
    }
    pub fn cannot_submit(&self) -> bool {
        self.form_errors.has_messages()
            || self.username.current().is_empty()
            || self.password.current().is_empty()
    }
}

#[inline_props]
pub fn PasswordInput<'a>(
    cx: Scope<'a>,
    state: UseState<String>,
    oninput: EventHandler<'a, FormEvent>,
) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "flex flex-col",
            label {
                r#for: "password",
                "Password"
            },
            input {
                id: "password",
                name: "password",
                class: "input-field",
                placeholder: "Password",
                r#type: "password",
                value: "{state.current()}",
                oninput: move |ev| oninput.call(ev)
            }
        }
    })
}

#[inline_props]
pub fn UsernameInput<'a>(
    cx: Scope<'a>,
    state: UseState<String>,
    oninput: EventHandler<'a, FormEvent>,
) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "flex flex-col",
            label {
                r#for: "username",
                "Username"
            },
            input {
                id: "username",
                name: "username",
                class: "input-field",
                placeholder: "User Name",
                value: "{state.current()}",
                oninput: move |ev| oninput.call(ev)
            }
        }
    })
}

fn LoginLink(cx: Scope) -> Element {
    cx.render(rsx! {
        Link {
            class: "link text-center",
            to: page::ACCOUNT_LOGIN,
            "Existing User Login"
        }
    })
}

pub fn Register(cx: Scope) -> Element {
    let api_client = ApiClient::global();
    let page_state = PageState::new(cx);
    let page_state = use_ref(cx, || page_state);
    let router = use_router(cx);
    let local_profile = use_local_profile(cx);

    let form_onsubmit = async_handler!(
        &cx,
        [api_client, page_state, router, local_profile],
        move |_| async move {
            use uchat_endpoint::user::endpoint::{CreateUser, CreateUserOk};
            let request_data = {
                use uchat_domain::{Password, Username};
                CreateUser {
                    username: Username::new(
                        page_state.with(|state| state.username.current().to_string()),
                    )
                    .unwrap(),
                    password: Password::new(
                        page_state.with(|state| state.password.current().to_string()),
                    )
                    .unwrap(),
                }
            };

            let response = fetch_json!(<CreateUserOk>, api_client, request_data);

            match response {
                Ok(res) => {
                    crate::util::cookie::set_session(
                        res.session_signature,
                        res.session_id,
                        res.session_expires,
                    );
                    router.navigate_to(page::HOME);
                    local_profile.write().user_id = Some(res.user_id);
                }
                Err(e) => page_state
                    .with_mut(|state| state.server_messages.set("register-fail", e.to_string())),
            }
        }
    );

    let username_oninput = sync_handler!([page_state], move |ev: FormEvent| {
        if let Err(err) = uchat_domain::Username::new(&ev.value) {
            page_state
                .with_mut(|state| state.form_errors.set("bad-username", err.formatted_error()));
        } else {
            page_state.with_mut(|state| state.form_errors.remove("bad-username"));
        }
        page_state.with_mut(|state| state.username.set(ev.value.clone()));
    });

    let password_oninput = sync_handler!([page_state], move |ev: FormEvent| {
        if let Err(err) = uchat_domain::Password::new(&ev.value) {
            page_state
                .with_mut(|state| state.form_errors.set("bad-password", err.formatted_error()));
        } else {
            page_state.with_mut(|state| state.form_errors.remove("bad-password"));
        }
        page_state.with_mut(|state| state.password.set(ev.value.clone()));
    });

    let submit_btn_style = maybe_class!(
        "btn-disabled",
        page_state.with(|state| state.cannot_submit())
    );

    cx.render(rsx! {
        form {
            class: "flex flex-col gap-5",
            prevent_default: "onsubmit",
            onsubmit: form_onsubmit,
            KeyedNotificationBox {
                legend: "Register Errors",
                notifications: page_state.with(|state| state.server_messages.clone())
            }
            UsernameInput {
                state: page_state.with(|state| state.username.clone()),
                oninput: username_oninput,
            }
            PasswordInput {
                state: page_state.with(|state| state.password.clone()),
                oninput: password_oninput,
            }
            LoginLink {}
            KeyedNotificationBox {
                legend: "Form Errors",
                notifications: page_state.with(|state| state.form_errors.clone())
            }
            button {
                class: "btn {submit_btn_style}",
                r#type: "submit",
                disabled: page_state.with(|state| state.cannot_submit()),
                "Signup"
            }
        }
    })
}
