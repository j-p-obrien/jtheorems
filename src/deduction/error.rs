#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JError {
    Illegal(&'static str),
    NameTaken(String),
}
