use std::{fmt::Debug, marker::PhantomData};

use leptos::*;
use uuid::Uuid;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum FormStatus {
    #[default]
    Prestine,
    Touched,
    Dirty,
}

pub trait AllSignalTraits<T>
where
    Self: SignalGet<Value = T>
        + SignalGetUntracked<Value = T>
        + SignalSet<Value = T>
        + SignalUpdate<Value = T>
        + SignalDispose
        + Clone
        + 'static,
    T: Debug + Default + PartialEq + Clone + 'static,
{
}

impl<T> AllSignalTraits<T> for RwSignal<T> where T: Debug + Default + PartialEq + Clone + 'static {}

#[derive(Clone, Copy, Debug)]
pub struct FormSignal<T, V, Rw, R, W>
where
    Rw: AllSignalTraits<T>,
    T: Debug + Default + PartialEq + Clone + 'static,
    V: Debug + Default + PartialEq + Clone + 'static,
    R: Fn(T) -> V + Clone + 'static,
    W: Fn(&mut T, V) + Clone + 'static,
{
    pub id: Uuid,
    src: Rw,
    memo: Memo<V>,
    writer: W,
    _reader: R,
    _phantom: PhantomData<V>,
    _phantom_t: PhantomData<T>,
}

impl<T, V, Rw, R, W> AllSignalTraits<V> for FormSignal<T, V, Rw, R, W>
where
    Rw: AllSignalTraits<T>,
    T: Debug + Default + PartialEq + Clone + 'static,
    V: Debug + Default + PartialEq + Clone + 'static,
    R: Fn(T) -> V + Clone + 'static,
    W: Fn(&mut T, V) + Clone + 'static,
{
}

impl<T, V, Rw, R, W> FormSignal<T, V, Rw, R, W>
where
    Rw: AllSignalTraits<T>,
    T: Debug + Default + PartialEq + Clone + 'static,
    V: Debug + Default + PartialEq + Clone + 'static,
    R: Fn(T) -> V + Clone + 'static,
    W: Fn(&mut T, V) + Clone + 'static,
{
    pub fn new(src: Rw, reader: R, writer: W) -> Self {
        let r = reader.clone();
        let m_src = src.clone();
        let memo = create_memo(move |_| r(m_src.get()));
        Self {
            id: Uuid::new_v4(),
            _phantom: PhantomData,
            _phantom_t: PhantomData,
            _reader: reader,
            memo,
            src,
            writer,
        }
    }

    pub fn clear(&self) {
        let d = V::default();
        self.set(d);
    }

    pub fn derive<V2, R2, W2>(
        &self,
        reader: R2,
        writer: W2,
    ) -> FormSignal<V, V2, FormSignal<T, V, Rw, R, W>, R2, W2>
    where
        V2: Debug + Default + PartialEq + Clone,
        R2: Fn(V) -> V2 + Clone + 'static,
        W2: Fn(&mut V, V2) + Clone + 'static,
    {
        FormSignal::new(self.clone(), reader, writer)
    }
}

impl<T, V, Rw, R, W> SignalSet for FormSignal<T, V, Rw, R, W>
where
    Rw: AllSignalTraits<T>,
    T: Debug + Default + PartialEq + Clone + 'static,
    V: Debug + Default + PartialEq + Clone + 'static,
    R: Fn(T) -> V + Clone + 'static,
    W: Fn(&mut T, V) + Clone + 'static,
{
    type Value = V;

    fn set(&self, new_value: Self::Value) {
        let old = self.try_get_untracked().unwrap_or_default();
        if old != new_value {
            let mut val = self.src.try_get_untracked().unwrap_or_default();
            let w = self.writer.clone();
            w(&mut val, new_value);
            self.src.set(val);
        }
    }

    fn try_set(&self, new_value: Self::Value) -> Option<Self::Value> {
        let old = self.try_get_untracked().unwrap_or_default();
        if old != new_value {
            let mut val = self.src.try_get_untracked().unwrap_or_default();
            let w = self.writer.clone();
            w(&mut val, new_value.clone());
            match self.src.try_set(val) {
                None => None,
                Some(_) => Some(new_value),
            }
        } else {
            None
        }
    }
}

impl<T, V, Rw, R, W> SignalUpdate for FormSignal<T, V, Rw, R, W>
where
    Rw: AllSignalTraits<T>,
    T: Debug + Default + PartialEq + Clone + 'static,
    V: Debug + Default + PartialEq + Clone + 'static,
    R: Fn(T) -> V + Clone + 'static,
    W: Fn(&mut T, V) + Clone + 'static,
{
    type Value = V;

    fn update(&self, f: impl FnOnce(&mut Self::Value)) {
        let old = self.try_get_untracked().unwrap_or_default();
        let mut inner = old.clone();
        f(&mut inner);
        if old != inner {
            self.src.update(|value| {
                let w = self.writer.clone();
                w(value, inner);
            });
        }
    }

    fn try_update<O>(&self, f: impl FnOnce(&mut Self::Value) -> O) -> Option<O> {
        let old = self.try_get_untracked().unwrap_or_default();
        let mut inner = old.clone();
        let res = f(&mut inner);

        if inner != old {
            let w = self.writer.clone();
            self.src.try_update(|value| {
                w(value, inner);
                res
            })
        } else {
            Some(res)
        }
    }
}

impl<T, V, Rw, R, W> SignalGet for FormSignal<T, V, Rw, R, W>
where
    Rw: AllSignalTraits<T>,
    T: Debug + Default + PartialEq + Clone + 'static,
    V: Debug + Default + PartialEq + Clone + 'static,
    R: Fn(T) -> V + Clone + 'static,
    W: Fn(&mut T, V) + Clone + 'static,
{
    type Value = V;

    fn get(&self) -> Self::Value {
        self.memo.get()
    }

    fn try_get(&self) -> Option<Self::Value> {
        self.memo.try_get()
    }
}

impl<T, V, Rw, R, W> SignalGetUntracked for FormSignal<T, V, Rw, R, W>
where
    Rw: AllSignalTraits<T>,
    T: Debug + Default + PartialEq + Clone + 'static,
    V: Debug + Default + PartialEq + Clone,
    R: Fn(T) -> V + Clone + 'static,
    W: Fn(&mut T, V) + Clone + 'static,
{
    type Value = V;

    fn get_untracked(&self) -> Self::Value {
        self.memo.get_untracked()
    }

    fn try_get_untracked(&self) -> Option<Self::Value> {
        self.memo.try_get_untracked()
    }
}

impl<T, V, Rw, R, W> SignalDispose for FormSignal<T, V, Rw, R, W>
where
    Rw: AllSignalTraits<T>,
    T: Debug + Default + PartialEq + Clone + 'static,
    V: Debug + Default + PartialEq + Clone,
    R: Fn(T) -> V + Clone + 'static,
    W: Fn(&mut T, V) + Clone + 'static,
{
    fn dispose(self) {
        self.memo.dispose();
        self.src.dispose();
    }
}
