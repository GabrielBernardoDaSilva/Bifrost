#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum LifetimeSystemExec {
    OnBegin,
    OnUpdate,
    OnFinish
}