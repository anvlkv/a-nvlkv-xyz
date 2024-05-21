use form_signal::FormState;
use leptos::*;
use leptos_use::storage::StorageType;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, VariantArray};

use crate::app::Language;

use super::{worksheets::*, ProjectData};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub wk: WorkSheetsFormState,
    pub examples: Vec<ProjectData>,
    pub sequence: Vec<SeqStep>,
    pub storage_preference: FormState<Option<StorageMode>>,
    pub show_privacy_prompt: RwSignal<bool>,
    pub lang: Language,
}

#[derive(
    Serialize, Deserialize, Default, Display, EnumString, Debug, PartialEq, Eq, Clone, Copy,
)]
pub enum StorageMode {
    #[default]
    #[strum(to_string = "local")]
    Local,
    #[strum(to_string = "none")]
    None,
}

impl Into<StorageType> for &StorageMode {
    fn into(self) -> StorageType {
        match self {
            StorageMode::Local => StorageType::Local,
            StorageMode::None => StorageType::Session,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct SeqStep {
    pub href: String,
    pub process_step: ProcessStep,
    pub example: Option<String>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Display, VariantArray, Hash)]
pub enum ProcessStep {
    #[default]
    About = 0,
    Problem = 1,
    Solution = 2,
    Compromise = 3,
    Implement = 4,
    Iterate = 5,
    Inquire = 6,
}
