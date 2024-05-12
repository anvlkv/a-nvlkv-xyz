use leptos::*;
use uuid::Uuid;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum FormStatus {
    #[default]
    Prestine,
    Touched,
    Dirty,
}

/// Wrapper around leptos::RwSignal for working with multistep forms
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct FormState<T>
where
    T: Default + PartialEq + Clone + 'static,
{
    pub id: Uuid,
    value: RwSignal<T>,
    status: RwSignal<FormStatus>,
}

impl<T> SignalGet for FormState<T>
where
    T: Default + PartialEq + Clone + 'static,
{
    type Value = T;

    fn get(&self) -> T {
        self.value.get()
    }

    fn try_get(&self) -> Option<T> {
        self.value.try_get()
    }
}

impl<T> SignalGetUntracked for FormState<T>
where
    T: Default + PartialEq + Clone + 'static,
{
    type Value = T;

    fn get_untracked(&self) -> T {
        self.value.get_untracked()
    }

    fn try_get_untracked(&self) -> Option<T> {
        self.value.try_get_untracked()
    }
}

impl<T> SignalSet for FormState<T>
where
    T: Default + PartialEq + Clone + 'static,
{
    type Value = T;

    fn set(&self, value: T) {
        self.value.set(value);
        self.status.set(FormStatus::Dirty);
    }

    fn try_set(&self, value: T) -> Option<T> {
        if let Some(not) = self.value.try_set(value) {
            Some(not)
        } else {
            self.status.set(FormStatus::Dirty);
            None
        }
    }
}

impl<T> SignalUpdate for FormState<T>
where
    T: Default + PartialEq + Clone + 'static,
{
    type Value = T;

    fn update(&self, f: impl FnOnce(&mut T)) {
        self.value.update(|v| f(v));
        self.status.set(FormStatus::Dirty);
    }

    fn try_update<O>(&self, f: impl FnOnce(&mut T) -> O) -> Option<O> {
        if let Some(not) = self.value.try_update(|v| f(v)) {
            Some(not)
        } else {
            self.status.set(FormStatus::Dirty);
            None
        }
    }
}

impl<T> FormState<T>
where
    T: Default + PartialEq + Clone + 'static,
{
    pub fn new(value: T) -> Self {
        Self {
            id: Uuid::new_v4(),
            status: Default::default(),
            value: RwSignal::new(value),
        }
    }

    pub fn touch(&self) {
        self.status.update(|status| {
            if *status == FormStatus::Prestine {
                *status = FormStatus::Touched;
            }
        })
    }

    pub fn status(&self) -> FormStatus {
        self.status.get_untracked()
    }
}

impl<T> Default for FormState<T>
where
    T: Default + PartialEq + Clone + 'static,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}
