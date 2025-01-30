use super::types::TypePtr;

type Index = u32;

pub(crate) struct BoundVariable {
    index: Index,
    typ: TypePtr,
}

pub(crate) struct FreeVariable {
    index: Index,
    typ: TypePtr,
}
