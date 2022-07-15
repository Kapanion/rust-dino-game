use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoxCollider{
    half_size: Vec2,
    offset: Vec2,
}

impl BoxCollider{
    pub fn new(half_size: Vec2) -> BoxCollider {
        BoxCollider{
            half_size,
            offset: v2!(0.0, 0.0),
        }
    }

    pub fn with_offset(&self, offset: Vec2) -> BoxCollider{
        let mut new_collider = self.clone();
        new_collider.offset = offset;
        new_collider
    }

    fn contains_point(&self, pos: Vec2, point: Vec2) -> bool {
        point.x >= pos.x - self.half_size.x &&
        point.x <= pos.x + self.half_size.x &&
        point.y >= pos.y - self.half_size.y &&
        point.y <= pos.y + self.half_size.y
    }

    fn get_corners(&self, pos: Vec2) -> [Vec2; 4] {
        let mut arr = [pos; 4];
        arr[0].x -= self.half_size.x;  arr[0].y -= self.half_size.y;
        arr[1].x -= self.half_size.x;  arr[1].y += self.half_size.y;
        arr[2].x += self.half_size.x;  arr[2].y -= self.half_size.y;
        arr[3].x += self.half_size.x;  arr[3].y += self.half_size.y;
        arr
    }

    fn get_pos(&self, ecs: &ECS, entity_id: usize) -> Vec2{
        ecs.get_component::<Movable>(entity_id).unwrap().pos + self.offset
    }

    pub fn check_collision(ecs: &ECS, entity1: usize, entity2: usize) -> bool {
        let col1 = ecs.get_component::<BoxCollider>(entity1);
        if col1 == None {return false}
        let col1 = col1.unwrap();
        let col2 = ecs.get_component::<BoxCollider>(entity2);
        if col2 == None {return false}
        let col2 = col2.unwrap();
        let pos1 = col1.get_pos(ecs, entity1);
        let pos2 = col2.get_pos(ecs, entity2);
        for corner in col1.get_corners(pos1){
            if col2.contains_point(pos2, corner) {
                return true;
            }
        }
        for corner in col2.get_corners(pos2){
            if col1.contains_point(pos1, corner) {
                return true;
            }
        }
        false
    }

    pub fn get_bound_offset(&self, bound: BoundType) -> Vec2{
        match bound{
            BoundType::Up    => Vec2::new(0.0,  self.half_size.y),
            BoundType::Down  => Vec2::new(0.0, -self.half_size.y),
            BoundType::Left  => Vec2::new(-self.half_size.x, 0.0),
            BoundType::Right => Vec2::new( self.half_size.x, 0.0),
        }
    }

    pub fn get_bound(&self, ecs: &ECS, entity_id: usize, bound: BoundType) -> Vec2{
        self.get_pos(ecs, entity_id) + self.get_bound_offset(bound)
    }
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