use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use super::WorkSheets;

#[derive(Params, PartialEq, Clone)]
pub struct ExampleParams {
    pub example: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Example {
    pub id: String,
    pub wk: Option<WorkSheets>,
    pub title: String,
    pub description: String,
    pub translation_warning: bool,
    pub main_image_url: Option<String>,
    pub images: Vec<String>,
}
