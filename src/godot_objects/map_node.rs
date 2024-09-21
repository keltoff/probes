
use godot::prelude::*;
use godot::classes::mesh::PrimitiveType;
use godot::classes::{IMeshInstance3D, MeshInstance3D, SurfaceTool};
use crate::geometry::{OrientedPosition, Turnable};
use crate::{geometry::{Direction, Position}, map_container::{build_map, MapContainer}};
//use itertools::Itertools;

#[derive(GodotClass)]
#[class(base=MeshInstance3D)]
struct MapNode {
    map: Option<MapContainer<()>>,
    #[export] command_string: GString,
    base: Base<MeshInstance3D>
}

#[godot_api]
impl IMeshInstance3D for MapNode {
    fn init(base: Base<MeshInstance3D>) -> Self {
        godot_print!("Map node created"); // Prints to the Godot console
        
        Self {
            map: None,
            command_string: GString::default(),
            base,
        }
    }

    fn ready(&mut self) {
        self.build_mesh(self.command_string.to_string());
    }
}

#[godot_api]
impl MapNode {
    #[func]
    pub fn is_blocked(&self, pos: OrientedPosition) -> bool {
        match &self.map {
            Some(map) => !map.any(pos.position),
            None => false,
        }
    }

    #[func]
    pub fn is_walkable(&self, pos: OrientedPosition) -> bool {
        match &self.map {
            Some(map) => map.any(pos.position) && !map.any(pos.shifted(crate::geometry::Turn::Down).position),
            None => false,
        }
    }

    fn build_mesh(&mut self, command_line: String) {
        self.map = Some(build_map(command_line));

        let mut tool = SurfaceTool::new_gd();
        tool.begin(PrimitiveType::TRIANGLES);

        for position in self.map.as_ref().unwrap().positions() {
            self.make_room(position, &mut tool);
        }

        // commit
        tool.generate_normals();
        let mesh = tool.commit().unwrap();
        self.base_mut().set_mesh(mesh);

        //let material : Gd<StandardMaterial3D> = load("wall_material.tres");
        //self.base_mut().set_material_overlay(material);
    }

    fn make_room(&self, pos: Position, tool: &mut Gd<SurfaceTool>) {
        let dpos: Vector3 = pos.into();

        let radius = 0.5;
        let corners = [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let mut corners_rev = corners.clone();
        corners_rev.reverse();

        let map = self.map.as_ref().unwrap();
       
        // floor
        if !map.any(pos + Direction::Down.into()) {
            tool.set_color(Color::YELLOW);
            for (x, y) in corners {
                tool.set_color(Color::YELLOW);
                tool.set_uv(get_uv(x, y, Wall::Floor));
                tool.add_vertex(Vector3{x: -radius + x, y: -radius, z: - radius + y} + dpos);
            }
        }
        
        // sides
        if !map.any(pos + Direction::West.into()) {
            //tool.set_color(Color::GRAY);
            tool.set_color(Color::LIGHT_BLUE);
            for (x, y) in corners {
                tool.set_uv(get_uv(x, y, Wall::Side));
                tool.add_vertex(Vector3{x: -radius, y: -radius + y, z: - radius + x} + dpos);
            }
        }
        if !map.any(pos + Direction::East.into()) {
            // flipped
            tool.set_color(Color::LIGHT_GREEN);
            for (x, y) in corners_rev {
                tool.set_uv(get_uv(x, y, Wall::Side));
                tool.add_vertex(Vector3{x: radius, y: -radius + y, z: -radius + x} + dpos);
            }
        }
        if !map.any(pos + Direction::North.into()) {
            // flipped
            tool.set_color(Color::LIGHT_SALMON);
            for (x, y) in corners_rev {
                tool.set_uv(get_uv(x, y, Wall::Side));
                tool.add_vertex(Vector3{x: -radius + x, y: -radius + y, z: - radius} + dpos);
            }
        }
        if !map.any(pos + Direction::South.into()) {
            tool.set_color(Color::VIOLET);
            for (x, y) in corners {
                tool.set_uv(get_uv(x, y, Wall::Side));
                tool.add_vertex(Vector3{x: -radius + x, y: -radius +y, z: radius} + dpos);
            }
        }
        
        // ceiling
        if !map.any(pos + Direction::Up.into()) {
            tool.set_color(Color::IVORY);
            // flipped
            tool.set_color(Color::YELLOW);
            for (x, y) in corners_rev {
                tool.set_uv(get_uv(x, y, Wall::Ceiling));
                tool.add_vertex(Vector3{x: -radius + x, y: radius, z: -radius + y} + dpos);
            }
        }
    }
}

enum Wall {
    Side,
    Floor,
    Ceiling
}

fn get_uv(x: f32, y: f32, wall: Wall) -> Vector2 {
    // x, y is in 0..1 range
    // texture is 300x100
    // remap to 0..1 global, applying offset by wall

    let offset = match wall {
        Wall::Side => 0.,
        Wall::Floor => 1./3.,
        Wall::Ceiling => 2./3.,
    };

    Vector2 { x: offset + x / 3., y: y }
}
