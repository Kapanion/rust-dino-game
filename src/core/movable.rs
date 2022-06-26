use glam::*;

pub struct Movable{
    pub pos: Vec2,
    pub velocity: Vec2,
    pub gravity: Vec2,
}

impl Movable{
    pub fn new(pos: Vec2) -> Movable {
        Movable{
            pos:        pos,
            velocity:   Vec2::new(0.0, 0.0),
            gravity:    Vec2::new(0.0, 0.0),
        }
    }
}