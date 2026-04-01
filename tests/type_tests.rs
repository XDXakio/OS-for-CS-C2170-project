use programming_language::types::{type_of, Context, Type};
use programming_language::term::Term;

#[test]
fn test_bool_type() {
    let term = Term::True;
    let mut ctx = Context::new();

    let ty = type_of(&term, &mut ctx).unwrap();
    assert_eq!(ty, Type::Bool);
}