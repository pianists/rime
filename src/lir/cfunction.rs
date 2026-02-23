use crate::util::id_coll::Id;

use super::{cdata::{Struct, TaggedUnion}, ctypes::CType};

pub struct CVarId(pub usize);

pub struct CFunction {
    pub name: String,
    pub args: Vec<(CVarId, CType)>,
    pub ret_type: CType,
    pub body: Vec<Command>,
}

pub enum Command {
    Let {
        var: CVarId,
        ty: CType,
        expr: RhsExpr,
    },
    Switch {
        scrutinee: CVarId,
        result: CVarId,
        result_type: CType,
        cases: Vec<SwitchCase>,
    },
    Return(CVarId),
}

pub struct SwitchCase {
    pub variant: Id<Struct>,
    pub bindings: Vec<(CVarId, CType)>,
    pub ret: CVarId,
}

pub enum RhsExpr {
    Var(CVarId),
    Call { function: Id<CFunction>, args: Vec<CVarId> },
    CallClosure { closure: CVarId, args: Vec<CVarId> },
    ConsTaggedUnion { ty: Id<TaggedUnion>, variant: Id<Struct>, args: Vec<CVarId> },
    ConsClosure { function: Id<CFunction>, env_vars: Vec<CVarId> },
}
