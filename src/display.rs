use std::fmt::Display;

use crate::{ast::decode_nat, t, term::Term};
use Term::*;

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Attempt to decode as nat
        if let Some(n) = decode_nat(self) {
            return write!(f, "{n}");
        }
        match self {
            Var(x) => write!(f, "{x}"),
            Abs { var, body } => {
                write!(f, "{var} => {body}")
            }
            App(t1, t2) => {
                if matches!(**t1, Abs { .. }) {
                    write!(f, "({t1}) ")?;
                } else {
                    write!(f, "{t1} ")?;
                }

                if matches!(**t2, Abs { .. } | App(_, _) | Rec { .. }) {
                    write!(f, "({t2})")
                } else {
                    write!(f, "{t2}")
                }
            }
            True => write!(f, "true"),
            False => write!(f, "false"),
            Ite {
                cond,
                if_true,
                if_false,
            } => {
                write!(f, "if {cond} then {if_true} else {if_false}")
            }
            Zero => write!(f, "Z"),
            Succ(t) => write!(f, "S({t})"),
            Rec {
                scrutinee,
                if_zero,
                if_succ,
            } => {
                let aux = t!(rec !(*scrutinee.clone()) !(*if_zero.clone()) !(*if_succ.clone()));
                write!(f, "{aux}")
            }
        }
    }
}
