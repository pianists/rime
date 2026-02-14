use crate::util::id_coll::IdColl;

use super::types::Type;

pub struct Adt {
    pub name: String,
    pub variants: IdColl<Variant>,
}

pub struct Variant {
    pub name: String,
    pub fields: Vec<Type>,
}
