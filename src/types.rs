#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Bool,
    Nat,
    Func(Box<Type>, Box<Type>),
}