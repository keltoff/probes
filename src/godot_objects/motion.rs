use godot::builtin::Transform3D;

use crate::geometry::{OrientedPosition as Pos, Turn, Turnable};


pub struct Motion {
    from: Transform3D,
    to: Transform3D,
    length: f32,
    progress: f32,
    revert: bool,
}

impl Motion {
    pub fn forward_motion(pos: Pos, bonk: bool, length: f32) -> Motion {
        Motion{ from: pos.to_transform(), to: pos.shifted(Turn::Front).to_transform(), length, progress: 0., revert: bonk }
    }

    pub fn backward_motion(pos: Pos, bonk: bool, length: f32) -> Motion {
        Motion{ from: pos.to_transform(), to: pos.shifted(Turn::Back).to_transform(), length, progress: 0., revert: bonk }
    }

    pub fn turn_motion(pos: Pos, turn: Turn, length: f32) -> Motion {
        Motion { from: pos.to_transform(), to: pos.turned(turn).to_transform(), length, progress: 0., revert: false  }
    }

    pub fn finished(&self) -> bool {
        self.progress >= self.length
    }

    pub fn process(&mut self, step: f32) -> Transform3D {
        self.progress += step;
        if self.progress > self.length {
            self.to
        } else {
            let mut  progress = self.progress / self.length;
            if self.revert && progress > 0.5 {
                progress = 1. - progress;
            }

            self.from.interpolate_with(&self.to, progress)
        }
    }
}