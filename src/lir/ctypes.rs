use crate::util::id_coll::Id;

use super::cdata::{Enum, Struct, TaggedUnion};

pub enum CType {
    Void,
    Int,

    Enum(Id<Enum>),
    Struct(Id<Struct>),
    TaggedUnion(Id<TaggedUnion>),

    Ptr(Box<CType>),
    VoidPtr,

    FuncPtr { args: Vec<CType>, ret: Box<CType> },
}
