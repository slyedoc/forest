use super::linden_mayer::*;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use super::generate_mesh;

#[derive(Inspectable, Component, Debug, Copy, Clone)]
pub struct Abop19 {
    pub iterations: usize,
    pub r: f32,
    pub size: f32,
}
impl Default for Abop19 {
    fn default() -> Self {
        Self {
            iterations: 15,
            r: 1.46,
            size: 1.0,
        }
    }
}

impl From<Abop19> for Mesh {
    fn from(data: Abop19) -> Self {
        let axiom = vec![SymR::new_from_vec('A', vec![300.0]).unwrap()];

        let mut system = LMSystem::new();

        system.add_rule(Rule::new(
            'A',
            Cond::True,
            vec![
                SymExpr::new_from_vec('F', vec![Expr::Var(0)]).unwrap(),
                SymExpr::new_from_vec('[', vec![]).unwrap(),
                SymExpr::new_from_vec('+', vec![]).unwrap(),
                SymExpr::new_from_vec(
                    'A',
                    vec![Expr::Div(
                        Box::new(Expr::Var(0)),
                        Box::new(Expr::Const(data.r)),
                    )],
                )
                .unwrap(),
                SymExpr::new_from_vec(']', vec![]).unwrap(),
                SymExpr::new_from_vec('[', vec![]).unwrap(),
                SymExpr::new_from_vec('-', vec![]).unwrap(),
                SymExpr::new_from_vec(
                    'A',
                    vec![Expr::Div(
                        Box::new(Expr::Var(0)),
                        Box::new(Expr::Const(data.r)),
                    )],
                )
                .unwrap(),
                SymExpr::new_from_vec(']', vec![]).unwrap(),
            ],
            1,
        ));

        let (after, _iters) = system.develop(axiom, data.iterations);
        let t = build(&after, 0.0, 85.0, 10.0);
        generate_mesh(t, data.size)
    }
}
