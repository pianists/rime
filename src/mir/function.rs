use crate::util::id_coll::Id;

use super::{adt::{Adt, Variant}, types::Type};

pub struct VarId(pub usize);

pub struct Function {
    pub name: String,
    pub args: Vec<(VarId, Type)>,
    pub ret_type: Type,
    pub body: AExpr,
}

pub struct AExpr {
    pub lets: Vec<ALet>,
    pub ret: VarId,
}

pub struct ALet {
    pub var: VarId,
    pub command: ACommand,
}

pub enum ACommand {
    NewAdt { adt: Id<Adt>, variant: Id<Variant>, args: Vec<VarId> },
    Call { function: Id<Function>, args: Vec<VarId> },
    Match { var: VarId, cases: Vec<Case> },
}

pub struct Case {
    pub variant: Id<Variant>,
    pub bindings: Vec<VarId>,
    pub ret: VarId,
}
