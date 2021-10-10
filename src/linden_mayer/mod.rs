// L-Systems - https://en.wikipedia.org/wiki/L-system
//
// Credit to https://github.com/mneumann/lindenmayer-system
pub mod parametric;
pub mod turtle;
pub use expression::cond::Cond;
pub use expression_num::NumExpr as Expr;
pub use parametric::*;

use std::fmt::Debug;

use turtle::{Canvas, Turtle};

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

    // Determine extend of canvas
    // let mut bounds = Bounds::new();

    // // The EPS coordinates are from bottom to top, like turtle coordinates.
    // self.foreach_position(|pos| bounds.add_position(pos), 1.0, 1.0);

    // let (min_width, min_height) = (100.0, 100.0);
    // let width = bounds.width().max(min_width);
    // let height = bounds.height().max(min_height);
    // let border_percent = 0.1;

    // let scale = 1.0 + 2.0 * border_percent;

    // // use a stroke width of 0.1% of the width or height of the canvas
    // let stroke_width = scale * width.max(height) / 1000.0;
    // writeln!(wr, r#"{} setlinewidth"#, stroke_width)?;

    // for path in self.paths.iter() {
    //     if let Some((head, tail)) = path.split_first() {
    //         writeln!(wr, "newpath")?;
    //         writeln!(wr, "  {} {} moveto", head.0, head.1)?;
    //         for pos in tail {
    //             writeln!(wr, r#"  {} {} lineto"#, pos.0, pos.1)?;
    //         }
    //         writeln!(wr, r#"stroke"#)?;
    //     }
    // }
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
