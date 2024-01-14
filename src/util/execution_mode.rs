/// Used internally to differentiate between various configurable execution modes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExecutionMode {
    Sequential,
    Parallel,
}
