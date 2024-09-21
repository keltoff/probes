use godot::{builtin::GString, prelude::{Export, GodotConvert, Var}};


#[derive(Copy, Clone, Debug, PartialEq, Eq, GodotConvert, Var, Export)]
#[godot(via=GString)]
pub enum Turn {
    Left,
    Right,
    Up,
    Down,
    RollLeft,
    RollRight,
    Front,
    Back
}

pub trait Turnable: Sized {
    fn turned(&self, turn : Turn) -> Self;

    fn left(&self) -> Self {
        self.turned(Turn::Left)
    }

    fn right(&self) -> Self  {
        self.turned(Turn::Right)
    }

    fn up(&self) -> Self {
        self.turned(Turn::Up)
    }

    fn down(&self) -> Self  {
        self.turned(Turn::Down)
    }

    fn counter_clockwise(&self) -> Self  {
        self.turned(Turn::RollLeft)
    }

    fn clockwise(&self) -> Self  {
        self.turned(Turn::RollRight)
    }

    fn front(&self) -> Self  {
        self.turned(Turn::Front)
    }

    fn back(&self) -> Self  {
        self.turned(Turn::Back)
    }
}

pub trait MakeTurnable {

    fn make_turn(&mut self, turn : Turn);

    
    fn turn_left(&mut self) {
        self.make_turn(Turn::Left)
    }

    fn turn_right(&mut self) {
        self.make_turn(Turn::Right)
    }

    fn turn_up(&mut self){
        self.make_turn(Turn::Up)
    }

    fn turn_down(&mut self) {
        self.make_turn(Turn::Down)
    }

    fn turn_back(&mut self) {
        self.make_turn(Turn::Back)
    }

    fn roll_left(&mut self) {
        self.make_turn(Turn::RollLeft)
    }

    fn roll_right(&mut self) {
        self.make_turn(Turn::RollRight)
    }

}