mod option_model;
mod vec_model;

pub use option_model::{OptionModel, OptionModelWithValue};
use reactive_stores::{Field, StoreField};
pub use vec_model::{VecModel, VecModelWithValue};

use leptos::prelude::*;

pub struct Model<T, S = SyncStorage>
where
    T: 'static,
    S: Storage<T>,
{
    read: ModelRead<T, S>,
    write: ModelWrite<T, S>,
    on_write: Option<WriteSignal<T, S>>,
}

enum ModelRead<T, S>
where
    T: 'static,
    S: Storage<T>,
{
    Signal(Signal<T, S>),
    Field(Field<T, S>),
}

impl<T, S> Clone for ModelRead<T, S>
where
    T: 'static,
    S: Storage<T>,
{
    fn clone(&self) -> Self {
        match self {
            ModelRead::Signal(signal) => ModelRead::Signal(*signal),
            ModelRead::Field(field) => ModelRead::Field(*field),
        }
    }
}

impl<T, S> Copy for ModelRead<T, S> where S: Storage<T> {}

enum ModelWrite<T, S>
where
    T: 'static,
    S: Storage<T>,
{
    Signal(WriteSignal<T, S>),
    Field(Field<T, S>),
}

impl<T, S> Clone for ModelWrite<T, S>
where
    T: 'static,
    S: Storage<T>,
{
    fn clone(&self) -> Self {
        match self {
            ModelWrite::Signal(signal) => ModelWrite::Signal(*signal),
            ModelWrite::Field(field) => ModelWrite::Field(*field),
        }
    }
}

impl<T, S> Copy for ModelWrite<T, S> where S: Storage<T> {}

impl<T: Default + Send + Sync> Default for Model<T> {
    fn default() -> Self {
        RwSignal::new(Default::default()).into()
    }
}

impl<T, S> Clone for Model<T, S>
where
    S: Storage<T>,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, S> Copy for Model<T, S> where S: Storage<T> {}

impl<T: Send + Sync> Model<T> {
    fn new(value: T) -> Self {
        let rw_signal = RwSignal::new(value);
        rw_signal.into()
    }

    pub fn signal(&self) -> Signal<T>
    where
        T: Clone,
    {
        match self.read {
            ModelRead::Signal(signal) => signal,
            ModelRead::Field(field) => Signal::derive(move || field.get()),
        }
    }
}

impl<T, S> DefinedAt for Model<T, S>
where
    S: Storage<T>,
{
    fn defined_at(&self) -> Option<&'static std::panic::Location<'static>> {
        match self.read {
            ModelRead::Signal(signal) => signal.defined_at(),
            ModelRead::Field(field) => field.defined_at(),
        }
    }
}

impl<T: Send + Sync> With for Model<T> {
    type Value = T;

    fn try_with<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> Option<O> {
        match self.read {
            ModelRead::Signal(signal) => signal.try_with(f),
            ModelRead::Field(field) => field.try_with(f),
        }
    }
}

impl<T: Send + Sync> WithUntracked for Model<T> {
    type Value = T;

    fn try_with_untracked<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> Option<O> {
        match self.read {
            ModelRead::Signal(signal) => signal.try_with_untracked(f),
            ModelRead::Field(field) => field.try_with_untracked(f),
        }
    }
}

// TODO
impl<T: Send + Sync + Clone> Update for Model<T>
// where
//     Field<T>: Update<Value = T>,
{
    type Value = T;

    fn try_maybe_update<U>(&self, fun: impl FnOnce(&mut Self::Value) -> (bool, U)) -> Option<U> {
        let value = match self.write {
            ModelWrite::Signal(signal) => signal.try_maybe_update(fun),
            ModelWrite::Field(field) => field.writer().map(|mut field| fun(&mut *field).1), // TODO: This is wrong, and always updates the value regardless of the returned bool
        };

        if let Some(on_write) = self.on_write.as_ref() {
            let v = match self.read {
                ModelRead::Signal(signal) => signal.get_untracked(),
                ModelRead::Field(field) => field.get_untracked(),
            };
            on_write.set(v);
        }

        value
    }
}

impl<T, S> IsDisposed for Model<T, S>
where
    S: Storage<T>,
{
    fn is_disposed(&self) -> bool {
        match self.write {
            ModelWrite::Signal(signal) => signal.is_disposed(),
            ModelWrite::Field(field) => field.is_disposed(),
        }
    }
}

impl<T: Send + Sync> From<T> for Model<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: Send + Sync> From<RwSignal<T>> for Model<T> {
    fn from(rw_signal: RwSignal<T>) -> Self {
        let (read, write) = rw_signal.split();
        Self {
            read: ModelRead::Signal(read.into()),
            write: ModelWrite::Signal(write),
            on_write: None,
        }
    }
}

impl<T, S> From<Field<T, S>> for Model<T, S>
where
    S: Storage<T>,
{
    fn from(field: Field<T, S>) -> Self {
        Self {
            read: ModelRead::Field(field),
            write: ModelWrite::Field(field),
            on_write: None,
        }
    }
}

impl<T, S> From<(Signal<T, S>, WriteSignal<T, S>)> for Model<T, S>
where
    S: Storage<T>,
{
    fn from((read, write): (Signal<T, S>, WriteSignal<T, S>)) -> Self {
        Self {
            read: ModelRead::Signal(read),
            write: ModelWrite::Signal(write),
            on_write: None,
        }
    }
}

impl<T: Send + Sync> From<(ReadSignal<T>, WriteSignal<T>)> for Model<T> {
    fn from((read, write): (ReadSignal<T>, WriteSignal<T>)) -> Self {
        Self {
            read: ModelRead::Signal(read.into()),
            write: ModelWrite::Signal(write),
            on_write: None,
        }
    }
}

impl<T: Send + Sync> From<(Memo<T>, WriteSignal<T>)> for Model<T> {
    fn from((read, write): (Memo<T>, WriteSignal<T>)) -> Self {
        Self {
            read: ModelRead::Signal(read.into()),
            write: ModelWrite::Signal(write),
            on_write: None,
        }
    }
}

impl<T: Default + Send + Sync> From<(Option<T>, WriteSignal<T>)> for Model<T> {
    fn from((read, write): (Option<T>, WriteSignal<T>)) -> Self {
        let mut model = Self::new(read.unwrap_or_default());
        model.on_write = Some(write);
        model
    }
}

// TODO
// #[cfg(test)]
// mod test {
//     use super::Model;
//     use leptos::*;

//     #[test]
//     fn from() {
//         let runtime = create_runtime();

//         // T
//         let model: Model<i32> = 0.into();
//         assert_eq!(model.get_untracked(), 0);
//         model.set(1);
//         assert_eq!(model.get_untracked(), 1);

//         // RwSignal
//         let rw_signal = RwSignal::new(0);
//         let model: Model<i32> = rw_signal.into();
//         assert_eq!(model.get_untracked(), 0);
//         model.set(1);
//         assert_eq!(model.get_untracked(), 1);

//         // Read Write
//         let (read, write) = create_signal(0);
//         let model: Model<i32> = (read, write).into();
//         assert_eq!(model.get_untracked(), 0);
//         model.set(1);
//         assert_eq!(model.get_untracked(), 1);

//         runtime.dispose();
//     }
// }
