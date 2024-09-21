use super::Direction;
use super::MakeTurnable;
use super::Turnable;
use std::fmt;


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Orientation {
    forward: Direction,
    normal: Direction
    }

impl Orientation {
    pub fn new(fwd: Direction, normal: Direction) -> Orientation {
        Orientation{forward: fwd, normal: normal}
    }

    pub fn direction(&self) -> Direction {
        self.forward
    }

    pub fn normal(&self) -> Direction {
        self.normal
    }
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation{forward: Direction::North, normal: Direction::Up}
    }
}

impl Turnable for Orientation {
    fn turned(&self, turn : super::trait_turn::Turn) -> Self {
        match turn {
            super::trait_turn::Turn::Left => Orientation{forward: Direction::lt(&self.forward, &self.normal), normal: self.normal},
            super::trait_turn::Turn::Right => Orientation{forward: Direction::rt(&self.forward, &self.normal), normal: self.normal},
            super::trait_turn::Turn::Up => Orientation{forward: self.normal, normal: self.forward.rev()},
            super::trait_turn::Turn::Down => Orientation{forward: self.normal.rev(), normal: self.forward},
            super::trait_turn::Turn::RollLeft => Orientation{forward: self.forward, normal: Direction::lt(&self.forward, &self.normal)},
            super::trait_turn::Turn::RollRight =>  Orientation{forward: self.forward, normal: Direction::rt(&self.forward, &self.normal)},
            super::trait_turn::Turn::Front => self.clone(),
            super::trait_turn::Turn::Back => Orientation{forward: self.forward.rev(), normal: self.normal},
        }
    }
}

impl MakeTurnable for Orientation {
    fn make_turn(&mut self, turn : super::Turn) {
        let new_orientation = self.turned(turn);
        self.forward = new_orientation.forward;
        self.normal = new_orientation.normal;
    }
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}_{}", self.forward.to_string(), self.normal.to_string())
    }
}
