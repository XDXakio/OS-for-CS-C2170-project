use programming_language::{ ast::AST, module::Module, parser::{ parse_ast, parse_module }, term::{Term, nat }, types::{Context, Type, type_of} };

fn eval(ast: AST) -> Term {
    let module = Module::new_with_prelude();
    ast.desugar(&module).multistep()
}

#[test]
fn test_full_pipeline() {
    let module = Module::new_with_prelude();

    let (_, ast) = parse_ast(&module, "1 + 2").unwrap();
    let term = ast.desugar(&module);

    let mut ctx = Context::new();
    let ty = type_of(&term, &mut ctx).unwrap();

    assert_eq!(ty, Type::Nat);

    let result = term.multistep();
    assert_eq!(result, nat(3));
}

#[test]
fn test_type_and_eval_pipeline() {
    let module = Module::new_with_prelude();

    let (_, ast) = parse_ast(&module, "1 + 2").unwrap();
    let term = ast.desugar(&module);

    let mut ctx = Context::new();
    let ty = type_of(&term, &mut ctx).unwrap();

    assert_eq!(ty, Type::Nat);

    let result = term.multistep();
    assert_eq!(result, programming_language::term::nat(3));
}

#[test]
fn test_module_declarations() {
    // Input with spaces and newlines
    let input = "a = 1\nb = a";

    // Parse the module
    let (_, module) = parse_module(input).expect("Failed to parse module");

    // Get the AST for 'b' and desugar into a Term
    let ast_b = module.get_term_ast("b").expect("Module should contain 'b'");
    let term_b = ast_b.clone().desugar(&module);

    // Evaluate the term fully
    let result = term_b.multistep();

    // Expect that 'b' evaluates to 1
    assert_eq!(result, programming_language::term::nat(1));
}


#[test]
fn test_complex_arithmetic() {
    let result = eval(AST::Add(
        Box::new(AST::Mul(
            Box::new(AST::Nat(2)),
            Box::new(AST::Nat(3)),
        )),
        Box::new(AST::Nat(1)),
    ));

    assert_eq!(result.to_string(), "7");
}

#[test]
fn parse_and_eval_fst() {
    let module = Module::new_with_prelude();

    let input = "fst (0, true)";
    let (_, ast) = parse_ast(&module, input).unwrap();

    let term = ast.desugar(&module);
    let result = term.multistep();

    assert_eq!(result.to_string(), "0");
}

#[test]
fn parse_and_eval_snd() {
    let module = Module::new_with_prelude();

    let input = "snd (0, true)";
    let (_, ast) = parse_ast(&module, input).unwrap();

    let term = ast.desugar(&module);
    let result = term.multistep();

    assert_eq!(result.to_string(), "true");
}

#[test]
fn parse_pair_lambda() {
    let module = Module::new_with_prelude();

    let input = "fst ((x: Nat => x) 5, true)";
    let (_, ast) = parse_ast(&module, input).unwrap();

    let term = ast.desugar(&module);
    let result = term.multistep();

    assert_eq!(result.to_string(), "5");
}

#[test]
fn fst_non_pair_stuck() {
    let term = Term::Fst(Box::new(Term::Zero));
    let result = term.clone().multistep();

    // evaluation shouldn't crash
    assert_eq!(result, term);
}

#[test]
fn test_rec_zero() {
    let rec = AST::Rec {
        scrutinee: Box::new(AST::Nat(0)),
        if_zero: Box::new(AST::Nat(42)),
        if_succ: Box::new(AST::Add(
            Box::new(AST::Nat(1)),
            Box::new(AST::Var("x".to_string())),
        )),
    };
    let term = rec.desugar(&Module::new_with_prelude()).multistep();
    assert_eq!(term, nat(42));
}
/*
#[test]
fn test_rec_succ() {
    let rec = AST::Rec {
        scrutinee: Box::new(AST::Nat(2)),
        if_zero: Box::new(AST::Nat(0)),
        if_succ: Box::new(AST::Add(
            Box::new(AST::Nat(1)),
            Box::new(AST::Var("x".to_string())),
        )),
    };
    let term = rec.desugar(&Module::new_with_prelude()).multistep();
    // Should compute 2 + 1 + 0 = 3
    assert_eq!(term.to_string(), "3");
}
*/

#[test]
fn test_rec_factorial() {
    // factorial 3 using rec(n, 1, fun x acc => n * acc)
    let rec = AST::Rec {
        scrutinee: Box::new(AST::Nat(3)),
        if_zero: Box::new(AST::Nat(1)),
        if_succ: Box::new(AST::Mul(
            Box::new(AST::Var("x".to_string())), // n
            Box::new(AST::Var("y".to_string())), // result of recursive call
        )),
    };
    // For now we just check that evaluation doesn't crash
    let term = rec.desugar(&Module::new_with_prelude()).multistep();
    // We can’t easily typecheck yet if AST vars not bound
    println!("Rec factorial result: {:?}", term);
}

#[test]
fn test_parse_typecheck_eval_lambda() {
    let input = "(fun x: Nat => x + 1) 2";
    let module = Module::new_with_prelude();
    let (_, ast) = parse_ast(&module, input).unwrap();
    let term = ast.desugar(&module);
    
    // Typecheck
    let mut ctx = Context::new();
    let ty = type_of(&term, &mut ctx).unwrap();
    assert_eq!(ty, Type::Nat);

    // Eval
    let result = term.multistep();
    assert_eq!(result.to_string(), "3");
}
/*
#[test]
fn test_parse_typecheck_eval_list_lambda() {
    let input = "(fun x: Nat => [x, x + 1]) 2";
    let module = Module::new_with_prelude();
    let (_, ast) = parse_ast(&module, input).unwrap();
    let term = ast.desugar(&module);

    // Typecheck
    let mut ctx = Context::new();
    let ty = type_of(&term, &mut ctx).unwrap();
    assert_eq!(ty, Type::List(Box::new(Type::Nat)));

    // Eval
    let result = term.multistep();
    let elems = result.collect_list().unwrap();
    assert_eq!(elems.len(), 2);
    assert_eq!(elems[0].to_string(), "2");
    assert_eq!(elems[1].to_string(), "3");
}
*/

/*
#[test]
fn test_parse_typecheck_eval_pairs() {
    let input = "(fun x: Nat => (x, true)) 5";
    let module = Module::new_with_prelude();
    let (_, ast) = parse_ast(&module, input).unwrap();
    let term = ast.desugar(&module);

    let mut ctx = Context::new();
    let ty = type_of(&term, &mut ctx).unwrap();
    assert_eq!(ty.to_string(), "(Nat, Bool)");

    let result = term.multistep();

    match result {
        Term::Pair(ref head, ref tail) => {
            match (&**head, &**tail) {
                (Term::Pair(inner1, inner2), Term::True) => {
                    // inner1 should be Zero, inner2 should be True
                    match (&**inner1, &**inner2) {
                        (Term::Zero, Term::True) => (),
                        _ => panic!("Inner pair not (Zero, True): {:?}, {:?}", inner1, inner2),
                    }
                }
                _ => panic!("Unexpected pair result: {:?}", result),
            }
        }
        _ => panic!("Expected a pair, got {:?}", result),
    }
}
*/

#[test]
fn test_parse_nested_lambda_application() {
    let input = "(fun x: Nat => fun y: Nat => x + y) 2 3";
    let module = Module::new_with_prelude();
    let (_, ast) = parse_ast(&module, input).unwrap();
    let term = ast.desugar(&module);

    let mut ctx = Context::new();
    let ty = type_of(&term, &mut ctx).unwrap();
    assert_eq!(ty, Type::Nat);

    let result = term.multistep();
    assert_eq!(result.to_string(), "5");
}