/// Used internally to differentiate between various configurable execution modes.
#[derive(Clone, PartialEq, Eq)]
pub(crate) enum ExecutionMode {
    Sequential,
    Parallel,
}
