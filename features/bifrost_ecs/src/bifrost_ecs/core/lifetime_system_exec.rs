#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LifetimeSystemExec {
    OnBegin,
    OnUpdate,
    OnFinish
}