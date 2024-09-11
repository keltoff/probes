use core::f32;

use godot::builtin::{Transform3D, Vector3};

use crate::geometry::Direction;


impl Into<Vector3> for crate::geometry::Position {
    fn into(self) -> Vector3 {
        Vector3 { x: self.x as f32, y: self.z as f32, z: self.y as f32 }
    }
}

impl crate::geometry::Orientation {
    pub fn to_transform(&self) -> Transform3D {
        let ori = self;
        let zero = Transform3D::default();
        match ori.direction() {
            Direction::North => {
                let roll = match ori.normal() {
                Direction::North => todo!(),
                Direction::West => f32::consts::FRAC_PI_2,
                Direction::South => todo!(),
                Direction::East => -f32::consts::FRAC_PI_2,
                Direction::Up => 0.0,
                Direction::Down => f32::consts::PI,
            };
            zero.rotated(zero.basis.col_b(), -f32::consts::FRAC_PI_2)
                .rotated_local(zero.basis.col_a(), roll)
            },
            Direction::West => {
                let roll = match ori.normal() {
                Direction::North => -f32::consts::FRAC_PI_2,
                Direction::West => todo!(),
                Direction::South => f32::consts::FRAC_PI_2,
                Direction::East => todo!(),
                Direction::Up => 0.0,
                Direction::Down => f32::consts::PI,
            };
            zero.rotated(zero.basis.col_b(), 0.0)
                .rotated_local(zero.basis.col_a(), roll)
            },
            Direction::South => {
                let roll = match ori.normal() {
                Direction::North => todo!(),
                Direction::West => -f32::consts::FRAC_PI_2,
                Direction::South => todo!(),
                Direction::East => f32::consts::FRAC_PI_2,
                Direction::Up => 0.0,
                Direction::Down => f32::consts::PI,
            };
            zero.rotated(zero.basis.col_b(), f32::consts::FRAC_PI_2)
                .rotated_local(zero.basis.col_a(), roll)
        }
            Direction::East => {
                let roll = match ori.normal() {
                Direction::North => f32::consts::FRAC_PI_2,
                Direction::West => todo!(),
                Direction::South => -f32::consts::FRAC_PI_2,
                Direction::East => todo!(),
                Direction::Up => 0.0,
                Direction::Down => f32::consts::PI,
            };
            zero.rotated(zero.basis.col_b(), f32::consts::PI)
                .rotated_local(zero.basis.col_a(), roll)
            },
            Direction::Up => {
                let yaw = match ori.normal() {
                Direction::North => f32::consts::FRAC_PI_2,
                Direction::West => f32::consts::PI,
                Direction::South => -f32::consts::FRAC_PI_2,
                Direction::East => 0.0,
                Direction::Up => todo!(),
                Direction::Down => todo!(),
            };
            zero.rotated(zero.basis.col_b(), yaw)
                .rotated_local(zero.basis.col_c(), -f32::consts::FRAC_PI_2)
        },
            Direction::Down => {
                let yaw = match ori.normal() {
                    Direction::North => -f32::consts::FRAC_PI_2,
                    Direction::West => 0.0,
                    Direction::South => f32::consts::FRAC_PI_2,
                    Direction::East => f32::consts::PI,
                    Direction::Up => todo!(),
                    Direction::Down => todo!(),
            };
            zero.rotated(zero.basis.col_b(), yaw)
                .rotated_local(zero.basis.col_c(), f32::consts::FRAC_PI_2)
        },
        }
    }
}

impl crate::geometry::OrientedPosition {
    pub fn to_transform(&self) -> Transform3D {
        self.orientation
            .to_transform()
            .translated(self.position.into())
    }
}
