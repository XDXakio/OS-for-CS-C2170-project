use programming_language::types::{type_of, Context, Type};
use programming_language::term::Term;
use programming_language::ast::AST;
use programming_language::module::Module;

fn typecheck(ast: AST) -> Result<Type, String> {
    let module = Module::new_with_prelude();
    let term = ast.desugar(&module);
    let mut ctx = Context::new();

    type_of(&term, &mut ctx).map_err(|e| format!("{:?}", e))
}

#[test]
fn test_bool_type() {
    let term = Term::True;
    let mut ctx = Context::new();

    let ty = type_of(&term, &mut ctx).unwrap();
    assert_eq!(ty, Type::Bool);
}

#[test]
fn test_add_type_ok() {
    let ty = typecheck(AST::Add(
        Box::new(AST::Nat(1)),
        Box::new(AST::Nat(2)),
    )).unwrap();

    assert_eq!(ty, Type::Nat);
}

#[test]
fn test_add_type_error() {
    let result = typecheck(AST::Add(
        Box::new(AST::Nat(1)),
        Box::new(AST::True),
    ));

    assert!(result.is_err());
}

#[test]
fn test_mul_type_error() {
    let result = typecheck(AST::Mul(
        Box::new(AST::False),
        Box::new(AST::Nat(2)),
    ));

    assert!(result.is_err());
}

#[test]
fn test_eq_type_error() {
    let result = typecheck(AST::Eq(
        Box::new(AST::True),
        Box::new(AST::Nat(1)),
    ));

    assert!(result.is_err());
}