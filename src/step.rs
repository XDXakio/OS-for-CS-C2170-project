use crate::{term::Term};

use Term::*;

impl Term {
    pub fn whnf(t: &Term) -> Term {
        match t {
            App(l, r) => {
                let l_whnf = Term::whnf(l);
                match l_whnf {
                    Abs { var, body } => body.subst(&var, &*r.clone()),
                    other => App(Box::new(other), r.clone()),
                }
            }
            Var(v) => Var(v.clone()),
            Abs { var, body } => Abs { var: var.clone(), body: body.clone() },
            Ite { cond, if_true, if_false } => {
                let c_whnf = Term::whnf(cond);
                match c_whnf {
                    True => *if_true.clone(),
                    False => *if_false.clone(),
                    other => Term::Ite {
                        cond: Box::new(other),
                        if_true: if_true.clone(),
                        if_false: if_false.clone(),
                    }
                }
            }
            True => True,
            False => False,
            Zero => Zero,
            Succ(t1) => Succ(t1.clone()),
            Rec { scrutinee, if_zero, if_succ } => Rec {
                scrutinee: scrutinee.clone(),
                if_zero: if_zero.clone(),
                if_succ: if_succ.clone(),
            }
        }
    }
    /// Applies the `AppAbs` rule returning None if it doesn't apply.
    pub fn app_abs(&self) -> Option<Self> {
        match self {
            App(f, a) => {
                let f_whnf = Term::whnf(&*f);
                if let Abs { var, body } = f_whnf {
                    Some(body.subst(&var, &*a.clone()))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Applies the `App1` rule returning None if it doesn't apply.
    pub fn app1(&self) -> Option<Self> {
        match self {
            App(f, a) => {
                if let Some(f_step) = f.step() {
                    Some(App(Box::new(f_step), a.clone()))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Applies the `App2` rule returning None if it doesn't apply.
    pub fn app2(&self) -> Option<Self> {
        match self {
            App(f, a) => {
                if f.step().is_none() {
                    a.step().map(|a_step| App(f.clone(), Box::new(a_step)))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Applies the `Abs` rule returning None if it doesn't apply.
    pub fn abs(&self) -> Option<Self> {
        match self {
            Abs { var, body } => {
                body.step().map(|body_step| Abs { var: var.clone(), body: Box::new(body_step) })
            }
            _ => None,
        }
    }

    /// Applies the `Ite1` rule returning None if it doesn't apply.
    pub fn ite1(&self) -> Option<Self> {
        match self {
            Ite { cond, if_true, if_false } => {
                cond.step().map(|c_step| Ite { cond: Box::new(c_step), if_true: if_true.clone(), if_false: if_false.clone() })
            }
            _ => None,
        }
    }
    /// Applies the `Ite2` rule returning None if it doesn't apply.
    pub fn ite2(&self) -> Option<Self> {
        match self {
            Ite { cond, if_true, if_false } => {
                if Term::whnf(cond).step().is_none() && !matches!(Term::whnf(cond), True | False) {
                    if_true.step().map(|t_step| Ite { cond: cond.clone(), if_true: Box::new(t_step), if_false: if_false.clone() })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Applies the `Ite3` rule returning None if it doesn't apply.
    pub fn ite3(&self) -> Option<Self> {
        match self {
            Ite { cond, if_true, if_false } => {
                if Term::whnf(cond).step().is_none() && !matches!(Term::whnf(cond), True | False) && if_true.step().is_none() {
                    if_false.step().map(|t_step| Ite { cond: cond.clone(), if_true: if_true.clone(), if_false: Box::new(t_step) })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Applies `IteTrue` or `IteFalse` returning None if neither applies.
    pub fn ite(&self) -> Option<Self> {
        match self {
            Ite { cond, if_true, if_false } => {
                match Term::whnf(&*cond) {
                    True => Some(*if_true.clone()),
                    False => Some(*if_false.clone()),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Applies the `Succ1` rule returning None if it doesn't apply.
    pub fn succ1(&self) -> Option<Self> {
        match self { Succ(t) => t.step().map(|s| Succ(Box::new(s))), _ => None }
    }

    /// Applies the `Rec1` rule returning None if it doesn't apply.
    pub fn rec1(&self) -> Option<Self> {
        if let Succ(inner) = self {
            inner.step().map(|inner_step| Succ(Box::new(inner_step)))
        } else {
            None
        }
    }

    /// Applies the `Rec2` rule returning None if it doesn't apply.
    pub fn rec2(&self) -> Option<Self> {
        match self {
            Rec { scrutinee, if_zero, if_succ } => match &**scrutinee {
                Zero => Some(*if_zero.clone()),
                _ => None
            },
            _ => None
        }
    }

    /// Applies the `Rec3` rule returning None if it doesn't apply.
    pub fn rec3(&self) -> Option<Self> {
        match self {
            Rec { scrutinee, if_zero, if_succ } => match &**scrutinee {
                Succ(n) => Some(App(
                    Box::new(App(if_succ.clone(), n.clone())),
                    Box::new(Rec { scrutinee: n.clone(), if_zero: if_zero.clone(), if_succ: if_succ.clone() }),
                )),
                _ => None
            },
            _ => None
        }
    }

    /// Applies the `Rec` rule returning None if it doesn't apply.
    pub fn rec(&self) -> Option<Self> {
        if let Rec { scrutinee, if_zero, if_succ } = self {
            match Term::whnf(scrutinee) {
                Zero => Some(*if_zero.clone()),
                Succ(n) => Some(App(
                    Box::new(App(if_succ.clone(), n.clone())),
                    Box::new(Rec {
                        scrutinee: n.clone(),
                        if_zero: if_zero.clone(),
                        if_succ: if_succ.clone(),
                    }),
                )),
                other => scrutinee.step().map(|step| Rec {
                    scrutinee: Box::new(step),
                    if_zero: if_zero.clone(),
                    if_succ: if_succ.clone(),
                }),
            }
        } else {
            None
        }
    }

    /// Does a beta-reduction step returning None if no reduction rule applies.
    /// Note: `AppAbs`, `Ite` and `Rec` and `App1` should come before the other rules.
    pub fn step(&self) -> Option<Self> {
        self.app_abs()
            .or_else(|| self.ite())
            .or_else(|| self.rec())
            .or_else(|| self.app1())
            .or_else(|| self.ite1())
            .or_else(|| self.rec1())
            .or_else(|| self.succ1())
            .or_else(|| self.app2())
            .or_else(|| self.ite2())
            .or_else(|| self.rec2())
            .or_else(|| self.ite3())
            .or_else(|| self.rec3())
            .or_else(|| self.abs())
    }

    /// Does any number of beta-reduction steps.
    /// Returns the final term for which no reduction could be made.
    pub fn multistep(mut self) -> Self {
        while let Some(next) = self.step() {
            self = next;
        }
        self
    }

    /// Compares if two normalizing terms are beta-equivalent.
    pub fn beta_eq(&self, other: &Self) -> bool {
        let n1 = self.clone().multistep();
        let n2 = other.clone().multistep();
        n1 == n2
    }
}
