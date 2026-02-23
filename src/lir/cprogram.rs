use std::collections::btree_set::Union;

use crate::util::id_coll::IdColl;

use super::{cdata::{Enum, Struct, TaggedUnion}, cfunction::CFunction};

pub struct CProgram {
    pub enums: IdColl<Enum>,
    pub structs: IdColl<Struct>,
    pub tagged_unions: IdColl<TaggedUnion>,
    pub functions: IdColl<CFunction>,
}
