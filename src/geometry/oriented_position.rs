use godot::{builtin::Array, meta::{FromGodot, ToGodot}, prelude::{ConvertError, GodotClass, GodotConvert}};

use super::{MakeTurnable, Orientation, PosDelta, Position, Turn, Turnable};
use std::fmt::{self, Debug};

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

impl GodotConvert for OrientedPosition {
    type Via = Array<i32>;
}

impl FromGodot for OrientedPosition {
    fn try_from_godot(via: Self::Via) -> Result<Self, godot::prelude::ConvertError> {
        if via.len() == 5 {
            Result::Ok(OrientedPosition{ position: Position { x: via.at(0), y: via.at(1), z: via.at(2) },
                                         orientation: Orientation::new(super::Direction::try_from_godot(via.at(3))?,
                                                                       super::Direction::try_from_godot(via.at(4))?)})
        } else {
            Result::Err(ConvertError::new(format!("Godot struct (via) has bad length ({}, needs 5)", via.len())))
        }
    }
}

impl ToGodot for OrientedPosition {
    fn to_godot(&self) -> Self::Via {
        let mut result = Array::new();
        result.push(self.position.x);
        result.push(self.position.y);
        result.push(self.position.z);
        result.push(self.orientation.direction().to_godot());
        result.push(self.orientation.normal().to_godot());
        result
    }
}

