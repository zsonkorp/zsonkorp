#[derive(Debug, Eq, PartialEq)]
pub(crate) enum State {
    Setup,
    Started,
    Ended
}