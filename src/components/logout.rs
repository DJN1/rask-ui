#![allow(non_snake_case)]
use dioxus::{
    prelude::{
        dioxus_elements, fc_to_builder, format_args_f, rsx, Element, LazyNodes, NodeFactory, Scope,
        VNode,
    },
    router::Redirect,
};

pub fn Logout(cx: Scope) -> Element {
    cx.render(rsx! {
        p { class: "font-light text-black", "Logging out..." },
        Redirect { to: "/" }
    })
}
