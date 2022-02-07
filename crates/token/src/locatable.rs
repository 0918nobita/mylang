use crate::Range;

pub trait Locatable {
    fn locate(&self) -> Range;
}
