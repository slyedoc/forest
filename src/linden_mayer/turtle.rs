
use std::f32::consts::PI;
use std::ops::{Add, Neg};

use bevy::math::*;
use bevy::prelude::info;
use bevy_prototype_debug_lines::DebugLines;

#[derive(Copy, Clone, Debug)]
pub struct Position(f32, f32);

impl Position {
    pub fn origin() -> Position {
        Position(0.0, 0.0)
    }

    #[allow(dead_code)]
    pub fn min(&self, other: &Position) -> Position {
        Position(self.0.min(other.0), self.1.min(other.1))
    }

    #[allow(dead_code)]
    pub fn max(&self, other: &Position) -> Position {
        Position(self.0.max(other.0), self.1.max(other.1))
    }

    #[allow(dead_code)]
    pub fn min_max(&self, min_max: &(Position, Position)) -> (Position, Position) {
        (self.min(&min_max.0), self.max(&min_max.1))
    }
}

impl Add<Position> for Position {
    type Output = Position;
    fn add(self, other: Position) -> Self::Output {
        Position(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Degree(pub f32);

#[derive(Copy, Clone, Debug)]
pub struct Radiant(pub f32);

impl From<Radiant> for Degree {
    fn from(rad: Radiant) -> Self {
        Degree(rad.0 * 180.0 / PI)
    }
}

impl From<f32> for Degree {
    fn from(d: f32) -> Self {
        Degree(d)
    }
}

impl From<Degree> for Radiant {
    fn from(d: Degree) -> Self {
        Radiant(d.0 * PI / 180.0)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Distance(f32);

impl From<f32> for Distance {
    fn from(d: f32) -> Distance {
        Distance(d)
    }
}

impl Neg for Distance {
    type Output = Distance;
    fn neg(self) -> Self::Output {
        Distance(-self.0)
    }
}

impl Neg for Degree {
    type Output = Degree;
    fn neg(self) -> Self::Output {
        Degree(-self.0)
    }
}

pub trait Turtle {
    /// Move turtle forward by specified `distance`.
    fn forward<T: Into<Distance>>(&mut self, distance: T);

    /// Move turtle backward by specified `distance`.
    fn backward<T: Into<Distance>>(&mut self, distance: T) {
        self.forward(-distance.into())
    }

    /// Move turtle forward by specified `distance` *without* drawing.
    fn move_forward<T: Into<Distance>>(&mut self, distance: T);

    /// Rotate around `angle`. If `angle` is positive,
    /// the turtle is turned to the left, if negative,
    /// to the right.
    fn rotate<T: Into<Degree>>(&mut self, angle: T);

    /// Turn turtle right by `angle` degree.
    fn right<T: Into<Degree>>(&mut self, angle: T) {
        self.rotate(-angle.into());
    }

    /// Turn turtle left by `angle` degree.
    fn left<T: Into<Degree>>(&mut self, angle: T) {
        self.rotate(angle.into());
    }

    /// Returns `true` if pen is down.
    fn is_pen_down(&self) -> bool;

    /// Returns `true` if pen is up.
    fn is_pen_up(&self) -> bool {
        !self.is_pen_down()
    }

    /// Put the pen down.
    fn pen_down(&mut self);

    /// Put the pen up.
    fn pen_up(&mut self);

    fn goto(&mut self, pos: Position);

    fn home(&mut self) {
        self.goto(Position::origin());
    }

    /// Push current turtle state on stack.
    fn push(&mut self);

    /// Restore previously saved turtle state.
    fn pop(&mut self);
}

#[derive(Clone)]
struct TurtleState {
    pos: Position,
    angle: Degree,
    pendown: bool,
}

pub struct Canvas {
    states: Vec<TurtleState>,
    paths: Vec<Vec<Position>>,
}

impl Canvas {
    pub fn new() -> Canvas {
        let init_pos = Position::origin();
        let init_state = TurtleState {
            pos: init_pos,
            // The coordinate system we use: x from left to right. y from bottom to top.
            angle: Degree(0.0), // points upwards
            pendown: true,      // start with pen down
        };
        Canvas {
            states: vec![init_state],
            paths: vec![vec![init_pos]],
        }
    }

    #[inline]
    fn current_state_mut(&mut self) -> &mut TurtleState {
        self.states.last_mut().unwrap()
    }

    #[inline]
    fn current_state(&self) -> &TurtleState {
        self.states.last().unwrap()
    }

    #[inline]
    fn direction(&self, distance: Distance) -> (f32, f32) {
        let state = self.current_state();
        let rad: Radiant = state.angle.into();
        let (sin, cos) = rad.0.sin_cos();
        let dx = -sin * distance.0;
        let dy = cos * distance.0;
        (dx, dy)
    }

    fn line_to(&mut self, dst: Position) {
        self.paths.last_mut().unwrap().push(dst);
    }

    fn move_to(&mut self, dst: Position) {
        if self.paths.is_empty() {
            self.paths.push(vec![dst]);
        } else {
            let begin_new_path = self.paths.last().unwrap().len() > 1;
            if begin_new_path {
                self.paths.push(vec![dst]);
            } else {
                // Replace first path element with current position
                self.paths.last_mut().unwrap()[0] = dst;
            }
        }
    }

    pub fn draw_lines(&self, lines: &mut DebugLines) {
        for path in self.paths.iter() {
            if let Some((head, tail)) = path.split_first() {
                for pos in tail {
                    info!("{:?}", pos);
                    lines.line(vec3( head.0, head.1, 0.0), vec3(pos.0, pos.1, 0.0), 30.0);
                }
            }
        }
    }
}

impl Turtle for Canvas {
    /// Move turtle forward by specified `distance`.
    fn forward<T: Into<Distance>>(&mut self, distance: T) {
        let (dx, dy) = self.direction(distance.into());
        let src: Position = self.current_state().pos;
        let dst = Position(src.0 + dx, src.1 + dy);
        if self.is_pen_down() {
            self.line_to(dst);
        }
        self.current_state_mut().pos = dst;
    }

    fn rotate<T: Into<Degree>>(&mut self, angle: T) {
        let angle: Degree = angle.into();
        self.current_state_mut().angle.0 += angle.0;
    }

    fn move_forward<T: Into<Distance>>(&mut self, distance: T) {
        let (dx, dy) = self.direction(distance.into());
        let src: Position = self.current_state().pos;
        let dst = Position(src.0 + dx, src.1 + dy);
        self.move_to(dst);
        self.current_state_mut().pos = dst;
    }

    fn is_pen_down(&self) -> bool {
        self.current_state().pendown
    }

    /// Put the pen down.
    fn pen_down(&mut self) {
        let pos = self.current_state().pos;
        self.move_to(pos);
        self.current_state_mut().pendown = true;
    }

    /// Put the pen up.
    fn pen_up(&mut self) {
        self.current_state_mut().pendown = false;
    }

    /// Positions the turtle exactly at `position`.
    fn goto(&mut self, position: Position) {
        self.current_state_mut().pos = position;
        self.move_to(position);
    }

    /// Push current turtle state on stack.
    fn push(&mut self) {
        let state = self.current_state_mut().clone();
        self.states.push(state);
    }

    /// Restore previously saved turtle state.
    fn pop(&mut self) {
        self.states.pop();
        let pos = self.current_state().pos;
        self.move_to(pos);
    }
}
