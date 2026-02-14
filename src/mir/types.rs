use crate::util::id_coll::Id;

use super::{adt::Adt, function::Function};

pub enum Type {
    Adt { id: Id<Adt>, type_args: Vec<Type> },
    Function { id: Id<Function>, type_args: Vec<Type> },
}
