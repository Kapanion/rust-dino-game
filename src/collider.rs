use ggez::{GameResult, GameError};
use glam::*;

use crate::util::*;

pub enum Collider{
    BoxCollider(Vec2), // half size
    None
}

impl Collider{
    pub fn new_box(size: Vec2) -> Collider{
        Self::BoxCollider(size)
    }

    pub fn point_in_box(box_pos: Vec2, box_size: Vec2, point: Vec2) -> bool{
        point.x >= box_pos.x - box_size.x &&
        point.x <= box_pos.x + box_size.x &&
        point.y >= box_pos.y - box_size.y &&
        point.y <= box_pos.y + box_size.y
    }

    pub fn box_bound_offs(half_size: Vec2, bound: BoundType) -> Vec2{
        match bound{
            BoundType::Up    => Vec2::new(0.0,  half_size.y),
            BoundType::Down  => Vec2::new(0.0, -half_size.y),
            BoundType::Left  => Vec2::new(-half_size.x, 0.0),
            BoundType::Right => Vec2::new( half_size.x, 0.0),
        }
    }

    pub fn get_corners(&self) -> GameResult<[Vec2; 4]>{
        match self{
            Collider::BoxCollider(col) =>{
                let mut arr = [Vec2::ZERO; 4];
                arr[0].x -= col.x;  arr[0].y -= col.y;
                arr[1].x -= col.x;  arr[1].y += col.y;
                arr[2].x += col.x;  arr[2].y -= col.y;
                arr[3].x += col.x;  arr[3].y += col.y;
                Ok(arr)
            }
            _ => {
                Err(GameError::CustomError(String::from("Actor::get_collider_corners() was called on an actor with no box collider.")))
            }
        }
    }

    pub fn contains_point(&self, point: Vec2) -> bool{
        match self{
            Collider::BoxCollider(col) => {
                point.x >= -col.x &&
                point.x <=  col.x &&
                point.y >= -col.y &&
                point.y <=  col.y
            }
            _ => false
        }
    }

    pub fn check_collision(&self, self_pos: Vec2, other: &Self, other_pos: Vec2) -> bool{
        match (self, other){
            (Collider::BoxCollider(col1), Collider::BoxCollider(col2)) => {
                Collider::point_in_box(self_pos, *col1 + *col2, other_pos)
            }
            _ => false
        }
    }
}