use crate::{intern::InternedString, list::List};
use num_rational::Rational64;
use std::fmt::Debug;

#[derive(Clone, PartialEq)]
pub enum Sexpr {
    Atom(Atom),
    Cons(Box<List<Sexpr>>),
}

impl Debug for Sexpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atom(a) => write!(f, "{:?}", a),
            Self::Cons(l) => write!(f, "{:?}", l),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Atom {
    Lit(Lit),
    Symbol(InternedString),
}

impl Debug for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lit(l) => write!(f, "{:?}", l),
            Self::Symbol(s) => write!(f, "{:?}", s),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Lit {
    Int(i64),
    Rational(Rational64),
    Real(f64),
    Char(char),
    String(InternedString),
}

impl Debug for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{:?}", i),
            Self::Rational(r) => write!(f, "{:?}", r),
            Self::Real(r) => write!(f, "{:?}", r),
            Self::Char(c) => write!(f, "{:?}", c),
            Self::String(s) => write!(f, "{:?}", s),
        }
    }
}
