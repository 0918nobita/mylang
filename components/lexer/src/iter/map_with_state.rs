//! 状態を持ち回りながら各要素をマッピングするアダプタ

pub struct MapWithState<I, S, F, B>
where
    I: Iterator + Sized,
    F: FnMut(&mut S, I::Item) -> B,
{
    state: S,
    iter: I,
    f: F,
}

impl<I, S, F, B> Iterator for MapWithState<I, S, F, B>
where
    I: Iterator,
    F: FnMut(&mut S, I::Item) -> B,
{
    type Item = B;

    fn next(&mut self) -> Option<B> {
        let a = self.iter.next()?;
        Some((self.f)(&mut self.state, a))
    }
}

pub trait MapWithStateExt: Iterator + Sized {
    fn map_with_state<S, F, B>(self, initial_state: S, f: F) -> MapWithState<Self, S, F, B>
    where
        F: FnMut(&mut S, Self::Item) -> B;
}

impl<I> MapWithStateExt for I
where
    I: Iterator,
{
    fn map_with_state<S, F, B>(self, initial_state: S, f: F) -> MapWithState<Self, S, F, B>
    where
        F: FnMut(&mut S, Self::Item) -> B,
    {
        MapWithState {
            state: initial_state,
            iter: self,
            f,
        }
    }
}
