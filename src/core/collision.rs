use std::rc::Rc;

use glam::*;
use crate::util::*;
use super::movable::{Movable};

pub struct BoxCollider{
    movable: Rc<Movable>,
    half_size: Vec2,
}

impl BoxCollider{
    fn new(movable: Movable, half_size: Vec2) -> BoxCollider {
        BoxCollider{
            movable: Rc::new(movable),
            half_size,
        }
    }

    fn contains_point(&self, point: Vec2) -> bool {
        point.x >= self.movable.pos.x - self.half_size.x &&
        point.x <= self.movable.pos.x + self.half_size.x &&
        point.y >= self.movable.pos.y - self.half_size.y &&
        point.y <= self.movable.pos.y + self.half_size.y
    }

    fn get_corners(&self) -> [Vec2; 4] {
        let mut arr = [self.movable.pos; 4];
        arr[0].x -= self.half_size.x;  arr[0].y -= self.half_size.y;
        arr[1].x -= self.half_size.x;  arr[1].y += self.half_size.y;
        arr[2].x += self.half_size.x;  arr[2].y -= self.half_size.y;
        arr[3].x += self.half_size.x;  arr[3].y += self.half_size.y;
        arr
    }

    fn check_collision(&self, other: &Self) -> bool {
        for corner in other.get_corners(){
            if self.contains_point(corner) {
                return true;
            }
        }
        for corner in self.get_corners(){
            if other.contains_point(corner) {
                return true;
            }
        }
        false
    }
}

mod tests {
    use super::*;
    use crate::v2;
    #[test]
    fn regular_overlap() {
        let tr1 = Rc::new(Movable::new(v2!(0.0, 0.0)));
        let tr2 = Rc::new(Movable::new(v2!(9.0, 9.0)));
        let col1 = BoxCollider{movable: tr1, half_size: v2!(5.0, 5.0)};
        let col2 = BoxCollider{movable: tr2, half_size: v2!(5.0, 5.0)};
        assert_eq!(col1.check_collision(&col2), true);
    }

    #[test]
    fn no_overlap() {
        let tr1 = Rc::new(Movable::new(v2!(0.0, 0.0)));
        let tr2 = Rc::new(Movable::new(v2!(9.0, 11.0)));
        let col1 = BoxCollider{movable: tr1, half_size: v2!(5.0, 5.0)};
        let col2 = BoxCollider{movable: tr2, half_size: v2!(5.0, 5.0)};
        assert_eq!(col1.check_collision(&col2), false);
    }

    #[test]
    fn one_box_inside() {
        let tr1 = Rc::new(Movable::new(v2!(0.0, 0.0)));
        let tr2 = Rc::new(Movable::new(v2!(0.0, 0.0)));
        let col1 = BoxCollider{movable: tr1, half_size: v2!(5.0, 5.0)};
        let col2 = BoxCollider{movable: tr2, half_size: v2!(15.0, 15.0)};
        assert_eq!(col1.check_collision(&col2), true);
    }
}