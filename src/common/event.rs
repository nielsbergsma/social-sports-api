use serde::Serialize;

pub trait Event: Serialize {
    fn kind(&self) -> &'static str;
}