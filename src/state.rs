mod cta;

pub use cta::CtaState;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum GlobalState<T> {
    Setup,
    Started(T), //T is intermediary states games may have once they have started
    Ended,
}