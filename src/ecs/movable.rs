use std::ops::Bound;
use std::rc::Rc;
use glam::*;

use crate::*;

use super::*;

// Components
use super::collision::{BoxCollider, BoundType};

pub struct Movable{
    pub pos: Vec2,
    pub velocity: Vec2,
    pub gravity: Vec2,
    // collider: Option<BoxCollider>,
}

impl Movable{
    pub fn get_pos(ecs: &ECS, entity_id: usize) -> Vec2 {
        // let mut id: usize = 0;
        // for component in ecs.borrow_component_vec::<Movable>().unwrap().iter_mut(){
        //     if entity_id == id{
        //         if let Some(component) = component{
        //             return component.pos;
        //         }
        //     }
        // }
        // panic!("Entity {} doesn't have a Movable component", entity_id);
        ecs.borrow_component::<Movable>(entity_id)
            .as_ref()
            .unwrap_or_else(||panic!("Entity {} doesn't have a Movable component", entity_id))
            .as_ref()
            .pos
    }

    pub fn new(pos: Vec2, velocity: Vec2, gravity: Vec2) -> Movable {
        Movable{
            pos,
            velocity,
            gravity,
            // collider: None,
        }
    }
    // pub fn add_collider(&mut self, collider: BoxCollider){
    //     self.collider = Some(collider);
    // }
    fn update_pos(&mut self, ecs: &ECS, entity_id: usize, dt: f32){
        self.velocity += self.gravity * dt;
        self.pos += self.velocity * dt;
        if let Some(mut collider) = ecs.borrow_component::<BoxCollider>(entity_id){
            self.check_ground_collision(ecs, entity_id, &mut collider);
        }
    }
    fn check_ground_collision(&mut self, ecs: &ECS, entity_id: usize, collider: &mut BoxCollider) {
        let lowest_point = collider.get_bound(ecs, entity_id, BoundType::Down).y;
        if lowest_point < GROUND_Y_COORD {
            self.pos.y += GROUND_Y_COORD - lowest_point;
            self.velocity = -self.velocity; //v2!(0.0, 0.0);
        }
    }

    // pub fn out_of_screen(&self) -> bool{
    //     let screen = BoxCollider::new(Vec2::ZERO, v2!(SCREEN.0 / 2.0, SCREEN.1 / 2.0));
    //     if let Some(collider) = self.collider.clone(){
    //         screen.check_collision(&collider)
    //     }
    //     else{
    //         screen.contains_point(self.pos)
    //     }
    // }
}

impl Component for Movable{
    fn update(&mut self, ecs: &ECS, entity_id: usize, dt: f32){
        self.update_pos(ecs, entity_id, dt);
    }
}
