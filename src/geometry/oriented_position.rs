use godot::prelude::GodotClass;

use super::{MakeTurnable, Orientation, PosDelta, Position, Turn, Turnable};
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, GodotClass)]
#[class(init)]
pub struct OrientedPosition {
    pub position: Position,
    pub orientation: Orientation
}

impl OrientedPosition {
    fn but(&self, orientation: Orientation) -> OrientedPosition {
        OrientedPosition{position: self.position, orientation: orientation}
    }

    pub fn shifted(&self, turn : Turn) -> Self {
        Self{position: self.position + PosDelta::from(self.orientation.turned(turn)), orientation: self.orientation}
    }

    pub fn forward(&mut self) {
        self.position += PosDelta::from(self.orientation);
    }

    pub fn backward(&mut self) {
        self.position -= PosDelta::from(self.orientation);
    }
}

impl Turnable for OrientedPosition {
    fn turned(&self, turn : Turn) -> Self {
        Self{position: self.position, orientation: self.orientation.turned(turn)}
    }
}

impl MakeTurnable for OrientedPosition {
    fn make_turn(&mut self, turn : Turn) {
        //println!("  Making turn from: {} with {:?}", self.orientation, turn);
        self.orientation.make_turn(turn);
        //println!("  Making turn to: {}", self.orientation);
    }
}

impl fmt::Display for OrientedPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.orientation, self.position)
    }
}

