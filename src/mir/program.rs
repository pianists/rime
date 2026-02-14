use super::adt::Adt;
use super::function::Function;

pub struct Program {
    pub adts: Vec<Adt>,
    pub functions: Vec<Function>,
}
