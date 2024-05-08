use leptos::*;
use uuid::Uuid;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum FormStatus {
    #[default]
    Prestine,
    Touched,
    Dirty,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormState<T: FormStateMember>
where
    T: Clone + 'static,
{
    pub id: Uuid,
    pub value: RwSignal<T>,
    pub status: RwSignal<FormStatus>,
    pub complete: RwSignal<bool>,
}

impl<T: FormStateMember> FormState<T>
where
    T: Clone + 'static,
{
    pub fn new(value: T) -> Self {
        Self {
            id: Uuid::new_v4(),
            status: Default::default(),
            complete: RwSignal::new(value.is_complete()),
            value: RwSignal::new(value),
        }
    }

    pub fn update<F>(&self, with: F)
    where
        F: FnOnce(&mut T),
    {
        self.value.update(|v| with(v));
        self.complete.set(self.value.get_untracked().is_complete());
        self.status.set(FormStatus::Dirty);
    }

    pub fn touch(&self) {
        self.status.update(|status| {
            if *status == FormStatus::Prestine {
                *status = FormStatus::Touched;
            }
        })
    }
}

pub trait FormStateMember {
    fn is_complete(&self) -> bool;
}

impl<T: FormStateMember> Default for FormState<T>
where
    T: Default + Clone + 'static,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> FormStateMember for T
where
    T: PartialEq + Default,
{
    /// default implementation with partial eq to Self::default()
    fn is_complete(&self) -> bool {
        self != &Self::default()
    }
}
