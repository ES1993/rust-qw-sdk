use std::any::TypeId;

use serde::de::DeserializeOwned;
use serde_json::Value;

pub fn parse_json<T: DeserializeOwned + 'static>(value: Value) -> serde_json::Result<T> {
    if TypeId::of::<T>() == TypeId::of::<()>() {
        Ok(unsafe { std::mem::transmute_copy(&()) })
    } else {
        serde_json::from_value(value)
    }
}
