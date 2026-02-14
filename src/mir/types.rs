pub struct AdtId(pub usize);

pub struct VariantId(pub usize);

pub struct FuncId(pub usize);

pub enum Type {
    Adt { id: AdtId, type_args: Vec<Type> },
    Function { id: FuncId, type_args: Vec<Type> },
}
