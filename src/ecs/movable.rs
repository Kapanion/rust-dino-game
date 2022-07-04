use glam::*;

use crate::*;

use super::*;

pub struct Movable{
    pub pos: Vec2,
    pub velocity: Vec2,
    pub gravity: Vec2,
}

impl Movable{
    pub fn new(pos: Vec2, velocity: Vec2, gravity: Vec2) -> Movable {
        Movable{
            pos,
            velocity,
            gravity,
        }
    }
    fn update_pos(&mut self, dt: f32){
        println!("Position updated");
        self.velocity += self.gravity * dt;
        self.pos += self.velocity * dt;
    }
}

impl Component for Movable{
    fn start(&mut self) {
        // todo!()
    }

    fn update(&mut self, dt: f32){
        self.update_pos(dt);
    }
}