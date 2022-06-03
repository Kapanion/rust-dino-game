use ggez::{event, GameError};
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use glam::*;

const GROUND_Y_COORD: f32 = -10.0;

/// Actor

pub enum ActorType {
    Player,
    Cactus,
}

pub struct Actor{
    pub tag: ActorType,
    pub pos: Vec2,
    pub velocity: Vec2,
    pub gravity: Vec2,
    pub collider: Collider,
}

impl Actor{
    pub fn new(tag: ActorType, pos: Vec2, velocity: Vec2, gravity: Vec2, collider: Collider) -> Actor{
        Actor{
            tag,
            pos,
            velocity,
            gravity,
            collider,
        }
    }

    // Position:

    pub fn update_pos(&mut self, dt: f32){
        self.velocity += self.gravity * dt;
        self.pos += self.velocity * dt;
        if self.pos.y < GROUND_Y_COORD{
            self.pos.y = GROUND_Y_COORD;
        }
    }


    // Collision stuff:

    fn get_collider_corners(&self) -> GameResult<[Vec2; 4]>{
        match self.collider{
            Collider::BoxCollider(col) =>{
                let mut arr = [self.pos; 4];
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

    fn point_inside_collider(&self, point: Vec2) -> bool {
        match self.collider{
            Collider::BoxCollider(col) => {
                point.x >= self.pos.x - col.x &&
                point.x <= self.pos.x + col.x &&
                point.y >= self.pos.y - col.y &&
                point.y <= self.pos.y + col.y
            }
            _ => false
        }
    }

    pub fn check_collision(&self, other: &Self) -> bool{
        let (col1, col2) = (&self.collider, &other.collider);
        match (col1, col2){
            (Collider::BoxCollider(col1), Collider::BoxCollider(col2)) => {
                Collider::point_in_box(self.pos, *col1 + *col2, other.pos)
            }
            _ => false
        }
    }
}

/// Collider

pub enum Collider{
    BoxCollider(Vec2),
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn box_collision_check_1() {
        let pos1 = Vec2::new(0.0, 0.0);
        let pos2 = Vec2::new(9.0, 9.0);
        let col1 = Collider::new_box(Vec2::new(5.0, 5.0));
        let col2 = Collider::new_box(Vec2::new(5.0, 5.0));
        let act1 = Actor::new(ActorType::Player, pos1, Vec2::new(0.0,0.0), Vec2::new(0.0,0.0), col1);
        let act2 = Actor::new(ActorType::Player, pos2, Vec2::new(0.0,0.0), Vec2::new(0.0,0.0), col2);
        assert_eq!(act1.check_collision(&act2), true);
    }
    #[test]
    fn box_collision_check_2() {
        let pos1 = Vec2::new(0.0, 0.0);
        let pos2 = Vec2::new(11.0, 9.0);
        let col1 = Collider::new_box(Vec2::new(5.0, 5.0));
        let col2 = Collider::new_box(Vec2::new(5.0, 5.0));
        let act1 = Actor::new(ActorType::Player, pos1, Vec2::new(0.0,0.0), Vec2::new(0.0,0.0), col1);
        let act2 = Actor::new(ActorType::Player, pos2, Vec2::new(0.0,0.0), Vec2::new(0.0,0.0), col2);
        assert_eq!(act1.check_collision(&act2), false);
    }
}