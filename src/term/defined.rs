#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DefinedData {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Defined {}

impl Defined {
    fn is_type(&self) -> bool {
        todo!()
    }
}
