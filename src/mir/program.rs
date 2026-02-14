use crate::util::id_coll::IdColl;

use super::adt::Adt;
use super::function::Function;

pub struct Program {
    pub adts: IdColl<Adt>,
    pub functions: IdColl<Function>,
}
