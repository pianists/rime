use crate::util::id_coll::{IdColl};

use super::ctypes::CType;

pub struct Enum {
    pub name: String,
    pub members: Vec<String>,
}

pub struct Struct {
    pub name: String,
    pub members: Vec<(String, CType)>,
}

pub struct TaggedUnion {
    pub name: String,
    pub variants: IdColl<Struct>,
}
