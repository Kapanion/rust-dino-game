use std::rc::Rc;
use glam::*;

use crate::*;

use super::*;

// Components
pub mod collision;
use collision::{BoxCollider, BoundType};

pub struct Movable{
    pub pos: Vec2,
    pub velocity: Vec2,
    pub gravity: Vec2,
    ground_check: bool, // not used for now
    collider: Option<BoxCollider>,
}

impl Movable{
    pub fn new(pos: Vec2, velocity: Vec2, gravity: Vec2) -> Movable {
        Movable{
            pos,
            velocity,
            gravity,
            ground_check: false,
            collider: None,
        }
    }
    // not used
    pub fn _ground_check_on(&mut self){
        self.ground_check = true;
    }
    pub fn add_collider(&mut self, collider: BoxCollider){
        self.collider = Some(collider);
    }
    fn update_pos(&mut self, dt: f32){
        self.velocity += self.gravity * dt;
        self.pos += self.velocity * dt;
        if let Some(mut collider) = self.collider.clone(){
            self.check_ground_collision(&collider);
            collider.update_pos(self.pos);
            self.collider = Some(collider);
        }
    }
    fn check_ground_collision(&mut self, collider: &BoxCollider) {
        if collider.get_bound(BoundType::Down).y < GROUND_Y_COORD {
            self.pos.y = GROUND_Y_COORD;
        }
    }
}

impl Component for Movable{
    fn update(&mut self, dt: f32){
        self.update_pos(dt);
    }
}
