use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;

#[macro_use]
mod components;

#[derive(PartialEq)]
pub enum FilterState {
    All,
    Incomplete,
    Complete,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Task {
    id: String,
    title: String,
    description: String,
    completed: bool,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize)]
pub struct Tasks(pub Vec<Task>);


fn main() {
    dotenv().ok();
    //println!("{}", env::var("API_BASE_URL").expect("API_BASE_URL must be set"));
    dioxus::web::launch(components::nav::nav_bar);
}
