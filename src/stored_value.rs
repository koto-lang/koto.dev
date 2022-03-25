use {
    crate::{get_local_storage_value, set_local_storage_value},
    std::{fmt::Display, ops::Deref, str::FromStr},
};

pub trait StorableValue: Default + Display + FromStr {}

impl<T: Default + Display + FromStr> StorableValue for T {}

pub struct StoredValue<T: StorableValue> {
    value: T,
    name: String,
}

impl<T: StorableValue> StoredValue<T> {
    pub fn new(name: &str) -> Self {
        Self::new_with_default(name, Default::default)
    }

    pub fn new_with_default(name: &str, default: impl FnOnce() -> T) -> Self {
        let value = match get_local_storage_value(name) {
            Some(value_string) => T::from_str(&value_string).unwrap_or_else(|_| default()),
            None => default(),
        };
        Self {
            value,
            name: name.into(),
        }
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
    }

    pub fn save(&self) {
        set_local_storage_value(&self.name, &self.value.to_string());
    }
}

impl<T: StorableValue> Deref for StoredValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
