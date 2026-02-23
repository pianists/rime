use crate::util::id_coll::Id;

use super::{adt::{Adt, Variant}, types::Type};

pub struct VarId(pub usize);

pub struct Function {
    pub name: String,
    pub captures: Vec<(VarId, Type)>,
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
    MakeClosure { function: Id<Function>, env: Vec<VarId> },
    Call { function: Id<Function>, args: Vec<VarId> },
    CallClosure { closure: VarId, args: Vec<VarId> },
    Match { scrutinee: VarId, cases: Vec<Case> },
}

pub struct Case {
    pub variant: Id<Variant>,
    pub bindings: Vec<VarId>,
    pub ret: VarId,
}
