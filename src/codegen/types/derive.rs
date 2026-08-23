use strum_macros::AsRefStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr)]
pub enum Derive {
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
}
