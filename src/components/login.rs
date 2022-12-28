#![allow(non_snake_case)]
use dioxus::{prelude::{
    dioxus_elements, format_args_f, rsx, Element, LazyNodes, NodeFactory, Scope, VNode,
}, events::FormEvent};
use reqwest::{Response, header::SET_COOKIE};
use wasm_bindgen::prelude::wasm_bindgen;
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Deserialize, Serialize)]
struct LoginPayload {
    email: String,
    password: String,
}

pub fn Login_form(cx: Scope) -> Element {
    let onsubmit = move |evt: FormEvent| {
        cx.spawn(async move {
            let payload = LoginPayload { email: evt.values["email"].clone(), password: evt.values["password"].clone() };
            let response = reqwest::Client::new()
                .post("http://127.0.0.1:5000/auth")
                .header("Content-Type", "application/json")
                .json(&payload)
                .send()
                .await;

            match response {
                Ok(data) => console_log!("{:?}", data.headers().get_all(SET_COOKIE)),
                Err(err) => console_log!("{err}"),
            }
        })
    };
    cx.render(rsx! {
        div { class: "grid place-items-center ", 
            div { class: "w-1/2 bg-white shadow-md border border-gray-200 rounded-lg p-4 sm:p-6 lg:p-8 dark:bg-gray-800 dark:border-gray-700",
                form { 
                    class: "space-y-6",
                    onsubmit: onsubmit,
                    prevent_default: "onsubmit",
                    h3 { class: "text-xl font-medium text-gray-900 dark:text-white", "Log in" },
                    div {
                        label { class: "text-sm font-emdium text-gray-900 mb-2 dark: text:gray-300", r#for: "email", "Email" },
                        input { class: "bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white", r#type: "email", name: "email", id: "email", placeholder: "mail@company.com", required: "" }
                    },
                    div {
                        label { class: "text-sm font-emdium text-gray-900 mb-2 dark: text:gray-300", r#for: "password", "Password" },
                        input { class: "bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white", r#type: "password", name: "password", id: "password", placeholder: "**********", required: "" }
                    },
                    button { class: "w-full text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800", r#type: "submit", "Log in" }
                }
            }
        }
    })
}
