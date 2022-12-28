#![allow(non_snake_case)]
use dioxus::prelude::{
    dioxus_elements, fc_to_builder, format_args_f, inline_props, rsx, use_state, Element,
    LazyNodes, NodeFactory, Props, Scope, VNode,
};
use dioxus::router::{Link, Route, Router};
use wasm_cookies;

use crate::components::login::Login_form;
use crate::components::logout::Logout;

fn login_cookie() -> Option<String> {
    match wasm_cookies::get("session") {
        Some(session_result) => match session_result {
            Ok(session_result) => return Some(session_result),
            Err(_) => return None,
        },
        None => return None,
    }
}

pub fn nav_bar(cx: Scope) -> Element {
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
    let logged_in_cookie = use_state(&cx, || login_cookie().unwrap_or_default());
    let links = if logged_in_cookie.is_empty() {
        rsx!(Link { to: "login", class: "font-light text-white", "Login" }, Link { to: "/signup", class: "font-light text-white", "Signup"})
    } else {
        rsx!(Link { to: "/logout", class: "font-light text-white", "Log out" })
    };
    let routes = if logged_in_cookie.is_empty() {
        rsx!(div { Route { to: "/login", Login_form {}}, Route { to: "/signup", Tester { tab: String::from("signup"), logged_in: logged_in_cookie.current().to_string() }}})
    } else {
        rsx!(Route { to: "/logout", Logout {}})
    };
    cx.render(rsx! {
        Router {
            div { class: "app mb-2",
                div { class: "bg-gray-900 mb-3",
                    nav { class: "container flex flex-row space-x-4 mx-auto py-4",
                        Link { to: "/", class: "text-center font-bold text-gray-200 hover:text-gray-100 transition", "Rask" },
                        div { class: "flex-1" },
                        links
                    }
                }
                div { class: "container mx-auto",
                    Route { to: "/", Tester { tab: String::from("home"), logged_in: logged_in_cookie.current().to_string() }},
                    routes
                }
            }
        }
    })
}

#[inline_props]
pub fn Tester(cx: Scope, tab: String, logged_in: String) -> Element {
    cx.render(rsx! {
        h1 { class: "text-5xl font-bold mt-0 mb-6", "tabname: {tab}" },
        p { class: "font-bold", "logged_in: {logged_in}" }
    })
}
