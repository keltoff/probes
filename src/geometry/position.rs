use std::ops;
use std::fmt;

use super::PosDelta;

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Default for Position {
    fn default() -> Self {
        Position{x: 0, y:0, z: 0}
    }
}

impl ops::Add<PosDelta> for Position {
    type Output = Position;

    fn add(self, rhs: PosDelta) -> Position {
        Position{x: self.x + rhs.dx, y: self.y + rhs.dy, z: self.z + rhs.dz}
    }
}

impl ops::Sub<Position> for Position {
    type Output = PosDelta;

    fn sub(self, rhs: Position) -> Self::Output {
        PosDelta{dx: self.x - rhs.x, dy: self.y - rhs.y, dz: self.x - rhs.z}
    }
}

impl ops::AddAssign<PosDelta> for Position {
    fn add_assign(&mut self, rhs: PosDelta) {
        self.x += rhs.dx;
        self.y += rhs.dy;
        self.z += rhs.dz;
    }
}

impl ops::SubAssign<PosDelta> for Position {
    fn sub_assign(&mut self, rhs: PosDelta) {
        self.x -= rhs.dx;
        self.y -= rhs.dy;
        self.z -= rhs.dz;
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}
