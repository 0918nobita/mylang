//! 状態を持ち回りながら各要素をマッピングするアダプタ

pub struct MapWithState<I, S, F, B>
where
    I: Iterator + Sized,
    S: Clone,
    F: Fn(S, I::Item) -> (S, B),
{
    state: S,
    iter: I,
    f: F,
}

impl<I, S, F, B> Iterator for MapWithState<I, S, F, B>
where
    I: Iterator,
    S: Clone,
    F: Fn(S, I::Item) -> (S, B),
{
    type Item = (S, B);

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.iter.next()?;
        let (next_state, res) = (self.f)(self.state.clone(), a);
        self.state = next_state.clone();
        Some((next_state, res))
    }
}

pub trait MapWithStateExt: Iterator + Sized {
    fn map_with_state<S, F, B>(self, initial_state: S, f: F) -> MapWithState<Self, S, F, B>
    where
        S: Clone,
        F: Fn(S, Self::Item) -> (S, B);
}

impl<I> MapWithStateExt for I
where
    I: Iterator,
{
    fn map_with_state<S, F, B>(self, initial_state: S, f: F) -> MapWithState<Self, S, F, B>
    where
        S: Clone,
        F: Fn(S, Self::Item) -> (S, B),
    {
        MapWithState {
            state: initial_state,
            iter: self,
            f,
        }
    }
}
