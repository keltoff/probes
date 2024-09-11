use super::{Direction, Orientation};
use std::ops::{Add, Sub, Mul};

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub struct PosDelta {
    pub dx: i32,
    pub dy: i32,
    pub dz: i32
}

impl Default for PosDelta {
    fn default() -> Self {
        Self{dx: 0, dy:0, dz: 0}
    }
}

impl From<Direction> for PosDelta {
    fn from(direction: Direction) -> PosDelta {
        match direction {
            Direction::North => PosDelta { dx: 0, dy: -1, dz: 0 },
            Direction::West => PosDelta { dx: -1, dy: 0, dz: 0 },
            Direction::South => PosDelta { dx: 0, dy: 1, dz: 0 },
            Direction::East => PosDelta { dx: 1, dy: 0, dz: 0 },
            Direction::Up => PosDelta { dx: 0, dy: 0, dz: 1 },
            Direction::Down => PosDelta { dx: 0, dy: 0, dz: -1 },
        }
    }
}

impl From<Orientation> for PosDelta {
    fn from(orientation: Orientation) -> PosDelta {
        Self::from(orientation.direction())
    }
}

impl Add for PosDelta {
    type Output = PosDelta;

    fn add(self, rhs: Self) -> Self::Output {
        Self{ dx: self.dx + rhs.dx, dy: self.dy + rhs.dy, dz: self.dz + rhs.dz }
    }
}

impl Sub for PosDelta {
    type Output = PosDelta;

    fn sub(self, rhs: Self) -> Self::Output {
        Self{ dx: self.dx - rhs.dx, dy: self.dy - rhs.dy, dz: self.dz - rhs.dz }
    }
}

impl Mul<i32> for PosDelta {
    type Output = PosDelta;

    fn mul(self, rhs: i32) -> Self::Output {
        Self{ dx: self.dx * rhs, dy: self.dy * rhs, dz: self.dz * rhs }
    }
}
