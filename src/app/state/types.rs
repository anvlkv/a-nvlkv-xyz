use form_signal::FormState;
use leptos::*;
use leptos_use::storage::StorageType;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, VariantArray};
use uuid::Uuid;

use crate::app::Language;

use super::worksheets::*;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub wk: WorkSheetsFormState,
    pub examples: Vec<Example>,
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

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Example {
    pub id: String,
    pub wk: Option<WorkSheets>,
    pub title: String,
    pub description: String,
    pub translation_warning: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct SeqStep {
    pub href: String,
    pub process_step: ProcessStep,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Display, VariantArray)]
pub enum ProcessStep {
    #[default]
    About,
    Problem,
    Solution,
    Compromise,
    Implement,
    Iterate,
    Inquire,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ExampleState<T> {
    pub id: Uuid,
    pub visited: bool,
    pub value: T,
}
