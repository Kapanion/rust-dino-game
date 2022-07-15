use std::rc::Rc;

use glam::*;
use crate::ecs::*;
use crate::util::*;
use crate::ecs::movable::{Movable};

#[derive(Clone)]
pub struct BoxCollider{
    pos: Vec2,
    half_size: Vec2,
}

impl BoxCollider{
    pub fn new(ecs: &mut ECS, entity_id: usize, half_size: Vec2) -> BoxCollider {
        let pos = Movable::get_pos(ecs, entity_id);
        BoxCollider{
            pos,
            half_size,
        }
    }

    fn contains_point(&self, point: Vec2) -> bool {
        point.x >= self.pos.x - self.half_size.x &&
        point.x <= self.pos.x + self.half_size.x &&
        point.y >= self.pos.y - self.half_size.y &&
        point.y <= self.pos.y + self.half_size.y
    }

    fn get_corners(&mut self, ecs: &mut ECS, entity_id: usize) -> [Vec2; 4] {
        self.update_pos(ecs, entity_id);
        let mut arr = [self.pos; 4];
        arr[0].x -= self.half_size.x;  arr[0].y -= self.half_size.y;
        arr[1].x -= self.half_size.x;  arr[1].y += self.half_size.y;
        arr[2].x += self.half_size.x;  arr[2].y -= self.half_size.y;
        arr[3].x += self.half_size.x;  arr[3].y += self.half_size.y;
        arr
    }

    pub fn check_collision(&mut self, ecs: &mut ECS, entity_id: usize, other: &mut Self) -> bool {
        for corner in other.get_corners(ecs, entity_id){
            if self.contains_point(corner) {
                return true;
            }
        }
        for corner in self.get_corners(ecs, entity_id){
            if other.contains_point(corner) {
                return true;
            }
        }
        false
    }

    fn get_bound_offset(&self, bound: BoundType) -> Vec2{
        match bound{
            BoundType::Up    => Vec2::new(0.0,  self.half_size.y),
            BoundType::Down  => Vec2::new(0.0, -self.half_size.y),
            BoundType::Left  => Vec2::new(-self.half_size.x, 0.0),
            BoundType::Right => Vec2::new( self.half_size.x, 0.0),
        }
    }

    pub fn get_bound(&mut self, ecs: &mut ECS, entity_id: usize, bound: BoundType) -> Vec2{
        self.update_pos(ecs, entity_id);
        self.pos + self.get_bound_offset(bound)
    }

    fn update_pos(&mut self, ecs: &mut ECS, entity_id: usize){
        self.pos = Movable::get_pos(ecs, entity_id);
    }
}

impl Component for BoxCollider{

}

#[derive(Clone, Copy)]
pub enum BoundType{
    Left,
    Right,
    Up,
    Down,
}
impl BoundType{
    pub fn horizontal(&self) -> bool{
        match self{
            BoundType::Left | BoundType::Right => true,
            _ => false,
        }
    }
    pub fn vertical(&self) -> bool{
        !self.horizontal()
    }
    pub fn opposite(&self) -> BoundType{
        match self{
            BoundType::Up    => BoundType::Down,
            BoundType::Down  => BoundType::Up,
            BoundType::Left  => BoundType::Right,
            BoundType::Right => BoundType::Left,
        }
    }
}


// mod tests {
//     use super::*;
//     use crate::v2;
//     #[test]
//     fn regular_overlap() {
//         let mut col1 = BoxCollider{pos: v2!(0.0, 0.0), half_size: v2!(5.0, 5.0)};
//         let mut col2 = BoxCollider{pos: v2!(9.0, 9.0), half_size: v2!(5.0, 5.0)};
//         assert_eq!(col1.check_collision(&mut col2), true);
//     }
//
//     #[test]
//     fn no_overlap() {
//         let mut col1 = BoxCollider{pos: v2!(0.0, 0.0), half_size: v2!(5.0, 5.0)};
//         let mut col2 = BoxCollider{pos: v2!(9.0, 11.0), half_size: v2!(5.0, 5.0)};
//         assert_eq!(col1.check_collision(&mut col2), false);
//     }
//
//     #[test]
//     fn one_box_inside() {
//         let mut col1 = BoxCollider{pos: v2!(0.0, 0.0), half_size: v2!(5.0, 5.0)};
//         let mut col2 = BoxCollider{pos: v2!(0.0, 0.0), half_size: v2!(15.0, 15.0)};
//         assert_eq!(col1.check_collision(&mut col2), true);
//     }
// }