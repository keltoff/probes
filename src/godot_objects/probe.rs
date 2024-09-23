
use godot::prelude::*;
use godot::classes::Node3D;
use crate::geometry::{MakeTurnable, OrientedPosition as Pos, Turn, Turnable};
use crate::godot_objects::motion::Motion;

#[derive(GodotClass)]
#[class(base=Node3D)]
struct Probe {
    position: Pos,
    motion: Option<Motion>,
    base: Base<Node3D>
}

#[godot_api]
impl INode3D for Probe {
    fn init(base: Base<Node3D>) -> Self {
        godot_print!("Probe created"); // Prints to the Godot console
        
        Self {
            position: Pos::default(),
            motion: None,
            base,
        }
    }

    fn process(&mut self, time: f64) {
        if self.motion.is_some() {
            let interpolated_transform = self.motion.as_mut().unwrap().process(time as f32);
            if self.motion.as_ref().unwrap().finished() {
                self.motion = None;
                self.update_position();
            } else {
                self.base_mut().set_transform(interpolated_transform);
            }
        }
    }
}

#[godot_api]
impl Probe {
    fn update_position(&mut self) {
        //godot_print!("Positon: {}", self.position); // Prints to the Godot console

        let transform = self.position.to_transform();
        self.base_mut().set_transform(transform);

        let pos = self.position.clone();
        self.base_mut().emit_signal("probe_on_position".into(), &["MeMe".to_variant(),
            pos.to_variant(),
            pos.position.x.to_variant(),
            pos.position.y.to_variant(),
            pos.position.z.to_variant()]);
    }

    #[func]
    fn target_pos(&self, turn: Turn) -> Pos {
        match turn {
            Turn::RollLeft | Turn::RollRight => self.position.turned(turn),
            _ => self.position.shifted(turn)
        }
    }

    #[func]
    fn move_smooth(&mut self, turn: Turn) {
        if self.motion.is_some() {
            return
        }

        let move_duration = 0.5; // s
        match turn {
            Turn::Left | Turn::Right | Turn::Up | Turn::Down | Turn::RollLeft | Turn::RollRight => {
                self.motion = Some(Motion::turn_motion(self.position, turn, move_duration));
                self.position.make_turn(turn);
            },
            Turn::Front => {
                self.motion = Some(Motion::forward_motion(self.position, false, move_duration));
                self.position.forward(); // needs to be after 
            },
            Turn::Back => {
                self.motion = Some(Motion::backward_motion(self.position, false, move_duration));
                self.position.backward(); // needs to be after 
            },
        }
    }

    #[signal]
    fn probe_on_position(name: GString, pos: Pos, x: i32, y: i32, z: i32);

    #[func]
    fn forward(&mut self) {
        self.move_smooth(Turn::Front);
    }

    #[func]
    fn backward(&mut self) {
        self.move_smooth(Turn::Back);
    }

    #[func]
    fn left(&mut self) {
        self.move_smooth(Turn::Left);
    }

    #[func]
    fn right(&mut self) {
        self.move_smooth(Turn::Right);
    }

    #[func]
    fn up(&mut self) {
        self.move_smooth(Turn::Up);      
    }

    #[func]
    fn down(&mut self) {
        self.move_smooth(Turn::Down);
    }

    #[func]
    fn roll_left(&mut self) {
        self.move_smooth(Turn::RollLeft);
    }

    #[func]
    fn roll_right(&mut self) {
        self.move_smooth(Turn::RollRight);
    }

    #[func]
    fn reset(&mut self) {
        self.position = Pos::default();
        self.update_position();
    }

    #[func]
    fn bonk(&mut self) {
        self.motion = Some(Motion::forward_motion(self.position, true, 0.5));
    }
}