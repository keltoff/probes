use std::iter::Iterator;

pub struct MapIterator<'a, RoomData> {
    u: u16,
    v: u16,
    u_limit: u16,
    v_limit: u16,
    map_get: &'a dyn Fn(u16,u16)->RoomData,
}

impl<'a, RoomData> MapIterator<'a, RoomData> {
    pub fn new(u_limit: u16, v_limit: u16, map_get : &'a dyn Fn(u16, u16)->RoomData) -> Self {
        MapIterator{ u: 0, v: 0, u_limit, v_limit, map_get}
    }
}

impl<'a, RoomData> Iterator for MapIterator<'a, RoomData> {
    type Item = (RoomData, bool);

    fn next(&mut self) -> Option<Self::Item> {
        self.u += 1;
        if self.u == self.u_limit {
            self.u = 0;
            self.v += 1;
        };
        if self.v >= self.v_limit {
            None
        } else {
            Some(((self.map_get)(self.u, self.v), self.u == 0))
        }
    }
}