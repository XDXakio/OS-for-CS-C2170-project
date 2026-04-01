use crate::{
    module::Module,
    t,
    term::{Term, nat},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AST {
    // Core lambda calculus
    Var(String),
    Abs {
        var: String,
        body: Box<AST>,
    },
    App(Box<AST>, Box<AST>),

    // Boolean fragment
    True,
    False,
    Ite {
        cond: Box<AST>,
        if_true: Box<AST>,
        if_false: Box<AST>,
    },

    // Natural number fragment
    Zero,
    Succ(Box<AST>),
    Rec {
        scrutinee: Box<AST>,
        if_zero: Box<AST>,
        if_succ: Box<AST>,
    },

    // Surface syntax sugar
    /// A name in the surrounding module
    Name(String),
    And(Box<AST>, Box<AST>),
    Or(Box<AST>, Box<AST>),
    Not(Box<AST>),
    Nat(u64),
    Add(Box<AST>, Box<AST>),
    Sub(Box<AST>, Box<AST>),
    Mul(Box<AST>, Box<AST>),
    Eq(Box<AST>, Box<AST>),
    Neq(Box<AST>, Box<AST>),
    Le(Box<AST>, Box<AST>),
    Lt(Box<AST>, Box<AST>),
    Ge(Box<AST>, Box<AST>),
    Gt(Box<AST>, Box<AST>),
}

use AST::*;

/// Attempts to decode a natural number term to an integer
pub fn decode_nat(mut t: &Term) -> Option<u64> {
    let mut n = 0;
    while let Term::Succ(t1) = t {
        t = &**t1;
        n += 1;
    }
    if let Term::Zero = t { Some(n) } else { None }
}

pub fn and() -> Term {
    let inner = Term::Ite {
        cond: Box::new(t!(b1)),
        if_true: Box::new(t!(b2)),
        if_false: Box::new(Term::False),
    };
    t!(b1 => b2 => !inner)
}

pub fn not() -> Term {
    let inner = Term::Ite {
        cond: Box::new(t!(a)),
        if_true: Box::new(Term::False),
        if_false: Box::new(Term::True),
    };
    t!(a => !inner)
}

pub fn or() -> Term {
    let inner = Term::Ite {
        cond: Box::new(t!(b1)),
        if_true: Box::new(Term::True),
        if_false: Box::new(t!(b2)),
    };
    t!(b1 => b2 => !inner)
}

/// The predecessor function for natural numbers
pub fn pred() -> Term {
    let body = Term::Rec {
        scrutinee: Box::new(t!(n)),
        if_zero: Box::new(Term::Zero),
        if_succ: Box::new(t!(pred => ih => pred)),
    };
    t!(n => !body)
}

pub fn plus() -> Term {
    let body = Term::Rec {
        scrutinee: Box::new(t!(n)),
        if_zero: Box::new(t!(m)),
        if_succ: Box::new(t!(pred => ih => !(Term::Succ(Box::new(t!(ih)))))),
    };
    t!(n => m => !body)
}

pub fn mult() -> Term {
    let body = Term::Rec {
        scrutinee: Box::new(t!(n)),
        if_zero: Box::new(Term::Zero),
        if_succ: Box::new(t!(pred => ih => !(plus()) m ih)),
    };
    t!(n => m => !body)
}

pub fn minus() -> Term {
    let body = Term::Rec {
        scrutinee: Box::new(t!(m)),
        if_zero: Box::new(t!(n)),
        if_succ: Box::new(t!(pred => ih => !(pred()) ih)),
    };
    t!(n => m => !body)
}

pub fn is_zero() -> Term {
    let body = Term::Rec {
        scrutinee: Box::new(t!(n)),
        if_zero: Box::new(Term::True),
        if_succ: Box::new(t!(pred => ih => !(Term::False))),
    };
    t!(n => !body)
}

pub fn eq() -> Term {
    t!(n => m => !(and()) (!(le()) n m) (!(le()) m n))
}

pub fn neq() -> Term {
    t!(n => m => !(not()) (!(eq()) n m))
}

pub fn le() -> Term {
    t!(n => m => !(is_zero()) (!(minus()) n m))
}

pub fn lt() -> Term {
    // Note: must use Succ rather than pred as 0 is not < 0
    t!(n => m => !(le()) !(Term::Succ(Box::new(t!(n)))) m)
}

pub fn ge() -> Term {
    t!(n => m => !(le()) m n)
}

pub fn gt() -> Term {
    t!(n => m => !(lt()) m n)
}

impl AST {
    pub fn desugar(self, env: &Module) -> Term {
        let d = |s: AST| s.desugar(env);

        match self {
            Var(x) => Term::Var(x),
            Abs { var, body } => Term::Abs {
                var,
                body: Box::new(d(*body)),
            },
            App(t1, t2) => t!(!(d(*t1)) !(d(*t2))),
            True => Term::True,
            False => Term::False,
            And(t1, t2) => t!(!(and()) !(d(*t1)) !(d(*t2))),
            Or(t1, t2) => t!(!(or()) !(d(*t1)) !(d(*t2))),
            Not(t) => t!(!(not()) !(d(*t))),
            Ite {
                cond,
                if_true,
                if_false,
            } => Term::Ite {
                cond: Box::new(d(*cond)),
                if_true: Box::new(d(*if_true)),
                if_false: Box::new(d(*if_false)),
            },
            Zero => Term::Zero,
            Succ(ast) => Term::Succ(Box::new(d(*ast))),
            Rec {
                scrutinee,
                if_zero,
                if_succ,
            } => Term::Rec {
                scrutinee: Box::new(d(*scrutinee)),
                if_zero: Box::new(d(*if_zero)),
                if_succ: Box::new(d(*if_succ)),
            },
            Nat(n) => nat(n),
            Add(t1, t2) => t!(!(plus()) !(d(*t1)) !(d(*t2))),
            Sub(t1, t2) => t!(!(minus()) !(d(*t1)) !(d(*t2))),
            Mul(t1, t2) => t!(!(mult()) !(d(*t1)) !(d(*t2))),
            Le(t1, t2) => t!(!(le()) !(d(*t1)) !(d(*t2))),
            Eq(t1, t2) => t!(!(eq()) !(d(*t1)) !(d(*t2))),
            Lt(t1, t2) => t!(!(lt()) !(d(*t1)) !(d(*t2))),
            Neq(t1, t2) => t!(!(neq()) !(d(*t1)) !(d(*t2))),
            Ge(t1, t2) => t!(!(ge()) !(d(*t1)) !(d(*t2))),
            Gt(t1, t2) => t!(!(gt()) !(d(*t1)) !(d(*t2))),
            Name(name) => env.get_term(&name).expect("env to contain name"),
        }
    }
}
