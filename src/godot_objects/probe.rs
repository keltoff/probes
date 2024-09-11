
use godot::prelude::*;
use godot::classes::Node3D;
use crate::geometry::{MakeTurnable, OrientedPosition as Pos};


#[derive(GodotClass)]
#[class(base=Node3D)]
struct Probe {
    position: Pos,
    base: Base<Node3D>
}

#[godot_api]
impl INode3D for Probe {
    fn init(base: Base<Node3D>) -> Self {
        godot_print!("Probe created"); // Prints to the Godot console
        
        Self {
            position: Pos::default(),
            //speed: 400.0,
            //angular_speed: std::f64::consts::PI,
            base,
        }
    }
}

#[godot_api]
impl Probe {
    fn update_position(&mut self) {
        //godot_print!("Positon: {}", self.position); // Prints to the Godot console

        let transform = self.position.to_transform();
        self.base_mut().set_transform(transform);
    }

    #[signal]
    fn position_changed();

    #[func]
    fn forward(&mut self) {
        self.position.forward();
        self.update_position();
    }

    #[func]
    fn left(&mut self) {
        self.position.turn_left();
        self.update_position();       
    }

    #[func]
    fn right(&mut self) {
        self.position.turn_right();
        self.update_position();
    }

    #[func]
    fn up(&mut self) {
        self.position.turn_up();
        self.update_position();       
    }

    #[func]
    fn down(&mut self) {
        self.position.turn_down();
        self.update_position();
    }

    #[func]
    fn reset(&mut self) {
        self.position = Pos::default();
        self.update_position();
    }
}