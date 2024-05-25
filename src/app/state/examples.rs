use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use super::WorkSheets;

#[derive(Params, PartialEq, Clone)]
pub struct ExampleParams {
    pub example: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectData {
    pub id: String,
    pub wk: Option<WorkSheets>,
    pub title: String,
    pub description: String,
    pub translation_warning: bool,
    pub main_image_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtendedProjectData {
    pub id: String,
    pub wk: Option<WorkSheets>,
    pub title: String,
    pub description: String,
    pub article: Vec<String>,
    pub translation_warning: bool,
    pub main_image_url: Option<String>,
    pub main_image_alt: Option<String>,
    pub images: Vec<String>,
}
