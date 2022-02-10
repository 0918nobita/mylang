use crate::Range;

/// ソースコード上で対応する範囲が特定できることを表現するトレイト
pub trait Locatable {
    /// 対応する範囲を返す
    fn locate(&self) -> Range;
}
