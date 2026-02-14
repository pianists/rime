use super::types::{AdtId, FuncId, Type, VariantId};

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
    NewAdt { adt: AdtId, variant: VariantId, args: Vec<VarId> },
    Call { function: FuncId, args: Vec<VarId> },
    Match { var: VarId, cases: Vec<Case> },
}

pub struct Case {
    pub variant: VariantId,
    pub bindings: Vec<VarId>,
    pub ret: VarId,
}
