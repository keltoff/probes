use std::{collections::HashMap, ops::Index};
use std::vec::Vec;
use crate::geometry::{MakeTurnable, OrientedPosition as Pos, PosDelta, Position, Range, Turnable};
//use crate::map_iterator::MapIterator;

#[derive(Default)]
pub struct MapContainer<RoomData> {
    data: HashMap<Position, RoomData>,
    x_range: Range,
    y_range: Range,
    z_range: Range,
}

impl<RoomData> MapContainer<RoomData> {
    fn set(&mut self, position : &Position, value : RoomData) {
        self.data.insert(*position, value);
        self.x_range.add(position.x);
        self.y_range.add(position.y);
        self.z_range.add(position.z);
    }

    pub fn get(&self, position : Position) -> Option<&RoomData> {
        self.data.get(&position)
    }

    pub fn any(&self, position : Position) -> bool {
        self.data.contains_key(&position)
    }

    pub fn get_xrange(&self) -> Range {
        self.x_range
    }

    pub fn get_yrange(&self) -> Range {
        self.y_range
    }

    pub fn get_zrange(&self) -> Range {
        self.z_range
    }
}

impl<RoomData> MapContainer<RoomData>  {
    /*
    pub fn for_slice(&self, pos: &Position, op : &dyn FnMut(Option<RoomData>) -> () ) {
        for x in self.x_range.iter() {
            for y in self.y_range.iter() {
                let pos = Position { x: x, y: y, z: pos.z };
                op(self.data.get(pos))
            }
        }
    }
     */

    pub fn slice_ortho(&self, pos: Position) {
        println!();
        println!("Map slice from {}", pos);
        for y in self.y_range.iter() {
            let map_line = self.x_range.iter()
                .map(|x|{self.any(Position{ x: x, y: y, z: pos.z })})
                .map(|b: bool|{match b {
                    true => '.',
                    false => '#',
                }}).collect::<String>();
            println!("{}", map_line);
        }
    }

    /*
    fn slice(&self, pos: Pos) {
        let line_iterator = match pos.orientation.direction() {
            Direction::North => 5..0,  //self.get_yrange().iter(),
            Direction::West => (0..3).rev(),//self.get_xrange().iter(),
            Direction::South => 0..5,  //self.get_yrange().iter().rev(),
            Direction::East => self.get_yrange().rev_iter(),
            Direction::Up => self.get_zrange().iter(),
            Direction::Down => self.get_yrange().rev_iter(),
        }
    }
     */

    fn print_tile(free: bool) {
        let tile_char = match free {
            true => '.',
            false => '#',
        };
        print!("{}", tile_char)
    }

    pub fn slice_around(&self, pos: Pos, radius: i32)
    {
        let left_shift = PosDelta::from(pos.orientation.left());
        let top_shift = PosDelta::from(pos.orientation);

        for dy in -radius..radius {
            for dx in -radius..radius {
                let target = pos.position + left_shift * dx + top_shift * dy ;
                MapContainer::<RoomData>::print_tile(self.any(target));
            }
        }
        
    }

    /*
    pub fn iter_around(&self, pos: Pos, radius: u16) -> MapIterator<Option<&RoomData>>
    {
        let left_shift = PosDelta::from(pos.orientation.left());
        let top_shift = PosDelta::from(pos.orientation);

        let map_access = move |u: u16, v:u16| {
            let target = pos.position + left_shift * (u-radius).into() + top_shift * (v - radius).into();
            self.data.get(&target)
        };
        
        MapIterator::new(2*radius+1, 2*radius+1, &map_access)
    }


    pub fn print_map_iter<T>(map_iter: MapIterator<Option<T>>) {
        for (x, line_end) in map_iter {
            print!("{}", match x.is_some() {true => '.', false => '#'});

            if line_end {
                println!();
            }
        }

    }
     */

    pub fn positions(&self) -> Vec<Position> {
        /*
        let cts = [m.get_xrange(), m.get_yrange(), m.get_zrange()];
        let mut iters = cts.map(|r| r.iter());
        //let mut multi = iters.iter_mut().multi_cartesian_product();
        let multi = (0..2).map(|_|crate::geometry::range::Range{start: 0, end: 10}.iter()).multi_cartesian_product();
            */

        let mut result = Vec::<Position>::new();

        for x in self.x_range.iter() {
            for y in self.y_range.iter() {
                for z in self.z_range.iter() {
                    let pos = Position{x, y: y, z: z};
                    if self.any(pos) {
                        result.push(pos);
                    }
                    
                }
            }
        }

        result
    }
}

impl<RoomData> Index<Position> for MapContainer<RoomData> {
    type Output = RoomData;

    fn index(&self, index: Position) -> &Self::Output {
        &self.data[&index]
    }
}

impl<RoomData> Index<Pos> for MapContainer<RoomData> {
    type Output = RoomData;

    fn index(&self, index: Pos) -> &Self::Output {
        self.index(index.position)
    }
}

pub(crate) fn build_map(command_string: String) -> MapContainer<()> {
    let mut result = MapContainer::default();
    let mut stack = Vec::<Pos>::default();
    let mut pos = Pos::default();
    result.set(&pos.position, ());

    let mut input_commands : Vec<char> = command_string.chars().rev().collect();

    while !input_commands.is_empty() {
        if let Some(command) = input_commands.pop() {
            match command {
                'l' => pos.turn_left(),
                'r' => pos.turn_right(),
                'u' => pos.turn_up(),
                'd' => pos.turn_down(),
                'f' => {
                    pos.forward();
                    result.set(&pos.position, ())
                },
                '1'..'9' => {
                    for _ in 0..command.to_digit(10).unwrap_or_default() {
                        pos.forward();
                        result.set(&pos.position, ())
                    }
                }
                '>'|'X' => stack.push(pos),
                '<' => {if !stack.is_empty() {pos = stack.pop().unwrap()}},
                _ => {println!("Warning: unknown command: {}", command);}
            }
            //println!("{}", pos);
        }

        

    }

    result
}
