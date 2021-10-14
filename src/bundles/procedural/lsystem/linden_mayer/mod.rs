// L-Systems - https://en.wikipedia.org/wiki/L-system
//
// Credit to https://github.com/mneumann/lindenmayer-system
mod parametric;
pub mod turtle;
pub use expression::cond::Cond;
pub use expression_num::NumExpr as Expr;
pub use parametric::*;

use std::fmt::Debug;

pub use turtle::*;

pub type Real = f32;
pub type SymExpr = PSym<char, Expr<Real>>;
pub type SymR = PSym<char, Real>;
pub type Rule = PRule<char, SymExpr, SymR, Cond<Expr<Real>>>;
pub type LMSystem = PSystem<Rule>;

/// Used to name symbols and variables.
pub trait Alphabet: Debug + PartialEq + Eq + Clone {}

impl Alphabet for &'static str {}
impl Alphabet for char {}
impl Alphabet for u8 {}
impl Alphabet for u16 {}
impl Alphabet for u32 {}
impl Alphabet for u64 {}
impl Alphabet for usize {}

/// An alphabet that distinguishes between terminal
/// and non-terminal symbols.
pub trait DualAlphabet: Alphabet {
    type Terminal;
    type NonTerminal: PartialOrd + Ord + Clone;

    /// If the character is a non-terminal, return Some(..). Otherwise return None.
    fn nonterminal(&self) -> Option<&Self::NonTerminal>;

    /// If the character is a terminal, return Some(..). Otherwise return None.
    fn terminal(&self) -> Option<&Self::Terminal>;
}

pub fn build(
    symstr: &[SymR],
    init_direction: f32,
    default_angle: f32,
    default_distance: f32,
) -> Canvas {
    let mut t = Canvas::default();
    t.right(init_direction);
    for sym in symstr.iter() {
        match (*sym.symbol(), sym.params().get(0)) {
            ('F', Some(&d)) => t.forward(d),
            ('F', None) => t.forward(default_distance),

            ('f', Some(&d)) => t.move_forward(d),
            ('f', None) => t.move_forward(default_distance),

            ('+', Some(&a)) => t.rotate(a),
            ('+', None) => t.rotate(default_angle),

            ('-', Some(&a)) => t.rotate(-a),
            ('-', None) => t.rotate(-default_angle),

            ('[', None) => t.push(),
            (']', None) => t.pop(),
            _ => {}
        }
    }
    t
}

#[allow(dead_code)]
pub fn symstr<S, R>(s: &str) -> Vec<S>
where
    S: ParametricSymbol<Sym = char, Param = R>,
    R: Clone + Debug + PartialEq,
{
    s.chars()
        .filter(|&c| !c.is_whitespace())
        .map(|c| S::new_from_vec(c, vec![]).unwrap())
        .collect()
}

#[allow(dead_code)]
pub fn rule(sym: char, successor: &str) -> Rule {
    Rule::new(sym, Cond::True, symstr(successor), 0)
}
