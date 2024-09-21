use godot::prelude::{Export, GodotConvert, Var};

#[derive(Copy, Clone, Debug, PartialEq, Eq, GodotConvert, Var, Export)]
#[godot(via=i32)]
pub enum Direction {
    North,
    West,
    South,
    East,
    Up,
    Down
}

impl Direction {
    pub fn rev(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }

    pub fn lt(fwd: &Direction, norm: &Direction) -> Direction {
        match (fwd, norm) {
            (Direction::North, Direction::North) => todo!(),
            (Direction::North, Direction::West) => Direction::Down,
            (Direction::North, Direction::South) => todo!(),
            (Direction::North, Direction::East) => Direction::Up,
            (Direction::North, Direction::Up) => Direction::West,
            (Direction::North, Direction::Down) => Direction::East,
            (Direction::West, Direction::North) => Direction::Up,
            (Direction::West, Direction::West) => todo!(),
            (Direction::West, Direction::South) => Direction::Down,
            (Direction::West, Direction::East) => todo!(),
            (Direction::West, Direction::Up) => Direction::South,
            (Direction::West, Direction::Down) => Direction::North,
            (Direction::South, Direction::North) => todo!(),
            (Direction::South, Direction::West) => Direction::Up,
            (Direction::South, Direction::South) => todo!(),
            (Direction::South, Direction::East) => Direction::Down,
            (Direction::South, Direction::Up) => Direction::East,
            (Direction::South, Direction::Down) => Direction::West,
            (Direction::East, Direction::North) => Direction::Down,
            (Direction::East, Direction::West) => todo!(),
            (Direction::East, Direction::South) => Direction::Up,
            (Direction::East, Direction::East) => todo!(),
            (Direction::East, Direction::Up) => Direction::North,
            (Direction::East, Direction::Down) => Direction::South,
            (Direction::Up, Direction::North) => Direction::East,
            (Direction::Up, Direction::West) => Direction::North,
            (Direction::Up, Direction::South) => Direction::West,
            (Direction::Up, Direction::East) => Direction::South,
            (Direction::Up, Direction::Up) => todo!(),
            (Direction::Up, Direction::Down) => todo!(),
            (Direction::Down, Direction::North) => Direction::West,
            (Direction::Down, Direction::West) => Direction::South,
            (Direction::Down, Direction::South) => Direction::East,
            (Direction::Down, Direction::East) => Direction::North,
            (Direction::Down, Direction::Up) => todo!(),
            (Direction::Down, Direction::Down) => todo!(),
        }
    }
    
    pub fn rt(fwd: &Direction, norm: &Direction) -> Direction {
        Direction::lt(fwd, norm).rev()
    }
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::North => "N".to_string(),
            Direction::West => "W".to_string(),
            Direction::South => "S".to_string(),
            Direction::East => "E".to_string(),
            Direction::Up => "U".to_string(),
            Direction::Down => "D".to_string(),
        }
    }
}

