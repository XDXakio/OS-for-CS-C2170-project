use crate::{ast::AST, term::Term};

use AST::*;

#[derive(Debug, Clone, Default)]
/// A module containing declarations
pub struct Module(Vec<(String, AST)>);

fn succ() -> AST {
    Abs {
        var: "n".to_string(),
        body: Box::new(Succ(Box::new(Var("n".to_string())))),
    }
}

fn rec() -> AST {
    Abs {
        var: "n".to_string(),
        body: Box::new(Abs {
            var: "z".to_string(),
            body: Box::new(Abs {
                var: "s".to_string(),
                body: Box::new(Rec {
                    scrutinee: Box::new(Var("n".to_string())),
                    if_zero: Box::new(Var("z".to_string())),
                    if_succ: Box::new(Var("s".to_string())),
                }),
            }),
        }),
    }
}

impl Module {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, name: String, ast: AST) {
        self.0.push((name, ast));
    }

    pub fn iter(&self) -> impl Iterator<Item = &(String, AST)> {
        self.0.iter()
    }

    pub fn new_with_prelude() -> Self {
        let mut m = Self::new();
        m.insert("succ".to_string(), succ());
        m.insert("rec".to_string(), rec());
        m
    }

    pub fn contains(&self, name: &str) -> bool {
        self.0.iter().rfind(|(n, _ast)| n == name).is_some()
    }

    pub fn get(&self, name: &str) -> Option<&AST> {
        self.0
            .iter()
            .rfind(|(n, _ast)| n == name)
            .map(|(_n, ast)| ast)
    }

    pub fn get_term(&self, name: &str) -> Option<Term> {
        // Note: this might accidentally reference a declaration that is defined later as the declaration names need not be unique.
        let ast = self.get(name).cloned()?;
        Some(ast.desugar(self))
    }
}
