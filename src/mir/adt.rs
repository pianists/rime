use super::types::Type;

pub struct Adt {
    pub name: String,
    pub variants: Vec<Variant>,
}

pub struct Variant {
    pub name: String,
    pub fields: Vec<Type>,
}
