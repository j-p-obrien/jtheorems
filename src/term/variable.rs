use super::types::TypePtr;

type Index = u32;

pub(super) struct BoundVariable {
    index: Index,
    typ: TypePtr,
}

pub(super) struct FreeVariable {
    typ: TypePtr,
}
