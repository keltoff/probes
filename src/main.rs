use geometry::Position;
use geometry::OrientedPosition as Pos;
use map_container::MapContainer;

mod geometry;
mod map_container;
mod map_iterator;

fn main() {
    let command = "3>l2<>r2<2>u3>l1<>r1<>u1<>d1<<1".to_string();
    let map = map_container::build_map(command);

    println!("Map size: {} x {} x {}", map.get_xrange(), map.get_yrange(), map.get_zrange());

    map.slice_ortho(Position::default());
    map.slice_ortho(Position{ x: 0, y: 0, z: 3});

    //println!();
    //MapContainer::<()>::print_map_iter(map.iter_around(Pos::default(), 5));

}
