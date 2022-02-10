use std::fmt::{Debug, Formatter, Result as FmtResult};

use serde::Serialize;

#[derive(PartialEq, Serialize)]
pub struct I32Entity(i32);

impl I32Entity {
    pub fn new(value: i32) -> Self {
        Self(value)
    }

    pub fn add(&self, rhs: &Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl Debug for I32Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

#[derive(PartialEq, Serialize)]
pub struct StrEntity(String);

impl StrEntity {
    pub fn new(value: String) -> Self {
        Self(value)
    }
}

impl Debug for StrEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize)]
pub enum Entity {
    I32(I32Entity),
    Str(StrEntity),
}
