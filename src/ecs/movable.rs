use crate::prelude::*;

use super::collision::{BoxCollider, BoundType};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Movable{
    pub pos: Vec2,
    pub velocity: Vec2,
    pub gravity: Vec2,
    pub on_ground: bool,
}

impl Movable{
    pub fn new(pos: Vec2, velocity: Vec2, gravity: Vec2) -> Movable {
        Movable {
            pos,
            velocity,
            gravity,
            on_ground: false,
        }
    }
    pub fn update_pos(ecs: &mut ECS, entity_id: usize, dt: f32){
        let new_mov: Option<Movable> = ecs.get_component(entity_id);
        if new_mov == None {return}
        let mut new_mov = new_mov.unwrap();
        new_mov.velocity += new_mov.gravity * dt;
        new_mov.pos += new_mov.velocity * dt;
        new_mov = Movable::check_ground_collision(ecs, entity_id, new_mov);
        ecs.set_component::<Movable>(entity_id, new_mov);
    }

    fn check_ground_collision(ecs: &ECS, entity_id: usize, mut mov: Movable) -> Movable{
        let col = ecs.get_component::<BoxCollider>(entity_id);
        if col == None {return mov}
        let col = col.unwrap();
        if !col.ground_check {return mov}
        let lowest_point_offs = col.get_bound_offset(BoundType::Down).y;
        let lowest_point = mov.pos.y + lowest_point_offs;
        if lowest_point < GROUND_Y_COORD {
            mov.pos.y = GROUND_Y_COORD - lowest_point_offs;
            mov.velocity = v2!(0.0, 0.0);
            mov.on_ground = true;
        }
        mov
    }

    pub fn jump(&mut self, vel: f32) {
        if !self.on_ground {return}
        self.velocity.y = vel;
        self.on_ground = false;
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
