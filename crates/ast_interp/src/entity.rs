//! 抽象構文木インタプリタで扱う「実体」の定義

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

/// 符号付き32ビット整数値
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

impl Display for I32Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

/// 文字列データ
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

impl Display for StrEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

/// 実行時型情報
#[derive(Debug)]
pub enum RuntimeTypeInfo {
    I32,
    Str,
}

impl Display for RuntimeTypeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                RuntimeTypeInfo::I32 => "i32",
                RuntimeTypeInfo::Str => "str",
            }
        )
    }
}

/// バイトコード実行時に扱う値
#[derive(Debug)]
pub enum Entity {
    I32(I32Entity),
    Str(StrEntity),
}

impl Entity {
    pub fn get_type(&self) -> RuntimeTypeInfo {
        match self {
            Entity::I32(_) => RuntimeTypeInfo::I32,
            Entity::Str(_) => RuntimeTypeInfo::Str,
        }
    }
}
