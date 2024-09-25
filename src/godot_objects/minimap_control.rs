
use std::cmp::max;

use godot::prelude::*;
use godot::classes::{Control, IControl};
use crate::geometry::OrientedPosition as Pos;
use crate::map_container::MapContainer;

#[derive(GodotClass)]
#[class(base=Control)]
struct MiniMapControl {
    origin_position: Pos,
    map_data: MapContainer<bool>,

    #[export]
    field_size: u16,

    base: Base<Control>
}

#[godot_api]
impl IControl for MiniMapControl {
    fn init(base: Base<Control>) -> Self {
        Self {
            origin_position: Pos::default(),
            field_size: 20,
            map_data: MapContainer::<bool>::default(),
            base,
        }
    }

    fn draw(&mut self) {
        let field_vec = Vector2::new(self.field_size.into(), self.field_size.into());
        let center = (self.base().get_size() - field_vec) / 2.;
        let radius = (center.x / self.field_size as f32) as i16 + 1;

        self.base_mut().draw_set_transform(center);


        for (u, v, room_data) in self.map_data.iter_around(self.origin_position, radius) {
            let rect = Rect2 { position: Vector2 { x: u as f32, y: v as f32 } * self.field_size as f32, size: field_vec };
            let color = match room_data {
                Some(true) => Color::DARK_GREEN,
                Some(false) => Color::DARK_GRAY,
                None => Color::BLACK,
            };
            self.base_mut().draw_rect(rect, color);
        };
    }
}

#[godot_api]
impl MiniMapControl {
    #[func]
    fn set_origin(&mut self, target: Pos) {
        self.origin_position = target;
        self.base_mut().queue_redraw();
    }

    #[func]
    fn set_map_square(&mut self, target: Pos, value: bool) {
        self.map_data.set(&target.position, value);
        self.base_mut().queue_redraw();
    }

}