use crate::collider::*;
use crate::util::*;
use crate::types_and_constants::*;
use crate::*;

use ggez::GameResult;
use glam::*;


pub enum ActorType {
    Dino,
    Cactus,
}

pub struct Actor{
    pub tag: ActorType,
    pub pos: Vec2,
    pub velocity: Vec2,
    pub gravity: Vec2,
    pub collider: Collider,
    pub in_air: bool,
}

impl Actor{
    pub fn new(tag: ActorType, pos: Vec2, velocity: Vec2, gravity: Vec2, collider: Collider) -> Actor{
        Actor{
            tag,
            pos,
            velocity,
            gravity,
            collider,
            in_air: false,
        }
    }

    // Position:

    pub fn update_pos(&mut self, dt: f32){
        self.velocity += self.gravity * dt;
        self.pos += self.velocity * dt;
        let offs = self.bound_offset(BoundType::Down);
        if self.pos.y + offs.y < GROUND_Y_COORD{
            self.pos.y = GROUND_Y_COORD - offs.y;
            self.velocity.y = 0.0;
            self.in_air = false;
        }
    }

    pub fn jump(&mut self, jump_speed: f32){
        self.velocity.y = jump_speed;
        self.in_air = true;
    }

    pub fn check_respawn_right(&mut self, screen_size: Screen2) -> bool{
        if self.out_of_screen_from(screen_size, BoundType::Left) {
            self.enter_screen_from(screen_size, BoundType::Right);
            true
        }
        else {false}
    }

    fn _out_of_screen_bounds(&self, screen_size: Screen2) -> bool{
        self.out_of_screen_from(screen_size, BoundType::Up)   ||
        self.out_of_screen_from(screen_size, BoundType::Down) ||
        self.out_of_screen_from(screen_size, BoundType::Left) ||
        self.out_of_screen_from(screen_size, BoundType::Right)
    }

    fn out_of_screen_from(&self, screen_size: Screen2, bound: BoundType) -> bool {
        let bound_pos = self.pos + self.bound_offset(bound.opposite());
        let bound_scr = screen_bound(screen_size, bound);
        match bound{
            BoundType::Left  => bound_pos.x < bound_scr.x,
            BoundType::Right => bound_pos.x > bound_scr.x,
            BoundType::Down  => bound_pos.y < bound_scr.y,
            BoundType::Up    => bound_pos.y > bound_scr.y,
        }
    }

    fn enter_screen_from(&mut self, screen_size: Screen2, bound: BoundType){
        self.pos = screen_bound(screen_size, bound) - self.bound_offset(bound.opposite());
    }


    // Collision stuff:
    fn _get_collider_corners(&self) -> GameResult<[Vec2; 4]>{
        let mut arr: [Vec2; 4] = self.collider.get_corners()?;
        for i in 0..4{
            arr[i] += self.pos;
        }
        Ok(arr)
    }
    
    fn _point_inside_collider(&self, point: Vec2) -> bool {
        self.collider.contains_point(point - self.pos)
    }

    pub fn check_collision(&self, other: &Self) -> bool{
        self.collider.check_collision(self.pos, &other.collider, other.pos)        
    }

    fn bound_offset(&self, bound: BoundType) -> Vec2{
        match self.collider{
            Collider::None => Vec2::new(0.0, 0.0),
            Collider::BoxCollider(col) => Collider::box_bound_offs(col, bound),
        }
    }
}

/// Tests

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn box_collision_check_1() {
        let pos1 = v2!(0.0, 0.0);
        let pos2 = v2!(9.0, 9.0);
        let col1 = Collider::new_box(v2!(5.0, 5.0));
        let col2 = Collider::new_box(v2!(5.0, 5.0));
        let act1 = Actor::new(ActorType::Dino, pos1, v2!(0.0,0.0), v2!(0.0,0.0), col1);
        let act2 = Actor::new(ActorType::Dino, pos2, v2!(0.0,0.0), v2!(0.0,0.0), col2);
        assert_eq!(act1.check_collision(&act2), true);
    }

    #[test]
    fn box_collision_check_2() {
        let pos1 = v2!(0.0, 0.0);
        let pos2 = v2!(11.0, 9.0);
        let col1 = Collider::new_box(v2!(5.0, 5.0));
        let col2 = Collider::new_box(v2!(5.0, 5.0));
        let act1 = Actor::new(ActorType::Dino, pos1, v2!(0.0,0.0), v2!(0.0,0.0), col1);
        let act2 = Actor::new(ActorType::Dino, pos2, v2!(0.0,0.0), v2!(0.0,0.0), col2);
        assert_eq!(act1.check_collision(&act2), false);
    }
}