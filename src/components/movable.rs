use crate::prelude::*;
use collision::BoundType;
use crate::KeyCode::End;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Movable{
    pub pos: Vec2,
    pub velocity: Vec2,
    pub gravity: Vec2,
    pub on_ground: bool,
    ground_check: bool,
}

impl Movable{
    pub fn new(pos: Vec2, velocity: Vec2, gravity: Vec2) -> Movable {
        Movable {
            pos,
            velocity,
            gravity,
            on_ground: false,
            ground_check: false,
        }
    }
    pub fn ground_check_on(&mut self){
        self.ground_check = true;
    }
    pub fn update_pos(ecs: &mut ECS, entity_id: usize, dt: f32){
        let new_mov: Option<Movable> = ecs.get_component(entity_id);
        if new_mov == None {return}
        let mut new_mov = new_mov.unwrap();
        new_mov.velocity += new_mov.gravity * dt;
        new_mov.pos += new_mov.velocity * dt;
        if new_mov.ground_check {
            new_mov = Movable::check_ground_collision(ecs, entity_id, new_mov);
        }
        ecs.set_component::<Movable>(entity_id, new_mov);
    }

    fn check_ground_collision(ecs: &ECS, entity_id: usize, mut mov: Movable) -> Movable{
        let col = ecs.get_component::<Collider>(entity_id);
        if col == None {return mov}
        let col = col.unwrap();
        let lowest_point_offs = col.get_bound_offset(BoundType::Down).y;
        let lowest_point = mov.pos.y + lowest_point_offs;
        if lowest_point < GROUND_Y_COORD {
            mov.pos.y = GROUND_Y_COORD - lowest_point_offs;
            mov.velocity = v2!(0.0, 0.0);
            mov.on_ground = true;
        }
        mov
    }

    // returns whether the jump was successful
    pub fn jump(&mut self, vel: f32) -> bool {
        if !self.on_ground {return false}
        self.velocity.y = vel;
        self.on_ground = false;
        true
    }
}

impl Update for Movable{
    fn update(ecs: &mut ECS, _assets: &Assets, _rng: &mut Rand32, entity_id: usize, _time: f32, dt: f32) {
        Movable::update_pos(ecs, entity_id, dt);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EndlessScroll{
    width: f32,
}

impl EndlessScroll{
    pub fn new(width: f32) -> EndlessScroll{
        EndlessScroll{
            width,
        }
    }
}

impl Update for EndlessScroll{
    fn update(ecs: &mut ECS, _assets: &Assets, _rng: &mut Rand32, entity_id: usize, _time: f32, _dt: f32) {
        let mut mov = ecs.get_component::<Movable>(entity_id).unwrap();
        let scroll = ecs.get_component::<EndlessScroll>(entity_id).unwrap();
        if mov.pos.x + scroll.width / 2.0 < -SCREEN.0 / 2.0 {
            mov.pos.x += scroll.width + SCREEN.0;
            ecs.set_component(entity_id, mov);
        }
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
