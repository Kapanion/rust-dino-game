// mod collider;

use std::ops::Bound;

use ggez::{event, GameError};
use ggez::graphics::{self, Color, DrawParam};
use ggez::{Context, GameResult};
use glam::*;

type Screen2 = (f32, f32);
// type Screen2 = Vec2;


const GROUND_Y_COORD: f32 = -10.0;
const JUMP_VELOCITY: f32 = 400.0;

/// Actor

#[derive(Clone, Copy)]
enum BoundType{
    Left,
    Right,
    Up,
    Down,
}
impl BoundType{
    fn horizontal(&self) -> bool{
        match self{
            BoundType::Left | BoundType::Right => true,
            _ => false,
        }
    }
    fn vertical(&self) -> bool{
        !self.horizontal()
    }
    fn opposite(&self) -> BoundType{
        match self{
            BoundType::Up    => BoundType::Down,
            BoundType::Down  => BoundType::Up,
            BoundType::Left  => BoundType::Right,
            BoundType::Right => BoundType::Left,
        }
    }
}

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

    fn jump(&mut self, jump_speed: f32){
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

    fn out_of_screen_bounds(&self, screen_size: Screen2) -> bool{
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
    fn get_collider_corners(&self) -> GameResult<[Vec2; 4]>{
        let mut arr: [Vec2; 4] = self.collider.get_corners()?;
        for i in 0..4{
            arr[i] += self.pos;
        }
        Ok(arr)
    }
    
    fn point_inside_collider(&self, point: Vec2) -> bool {
        self.collider.contains_point(point - self.pos)
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

    fn bound_offset(&self, bound: BoundType) -> Vec2{
        match self.collider{
            Collider::None => Vec2::new(0.0, 0.0),
            Collider::BoxCollider(col) => Collider::box_bound_offs(col, bound),
        }
    }
}

/// Collider

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

    fn box_bound_offs(half_size: Vec2, bound: BoundType) -> Vec2{
        match bound{
            BoundType::Up    => Vec2::new(0.0,  half_size.y),
            BoundType::Down  => Vec2::new(0.0, -half_size.y),
            BoundType::Left  => Vec2::new(-half_size.x, 0.0),
            BoundType::Right => Vec2::new( half_size.x, 0.0),
        }
    }

    fn get_corners(&self) -> GameResult<[Vec2; 4]>{
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

    fn contains_point(&self, point: Vec2) -> bool{
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
}

/// Input
#[derive(Debug)]
#[derive(Default)]
pub struct InputState{
    jump: bool,
}

impl InputState{
    pub fn jump_start(&mut self){
        self.jump = true;
    }
    pub fn jump_end(&mut self){
        self.jump = false;
    }
    pub fn jump(&self) -> bool{
        self.jump
    }
}

pub fn player_handle_input(actor: &mut Actor, input: &mut InputState, dt: f32) {
    if input.jump() && !actor.in_air {
        actor.jump(JUMP_VELOCITY);
        input.jump_end();
    }
}

/// World and screen positions

pub fn world_to_screen_coords(screen_size: Screen2, point: Vec2) -> Vec2 {
    let x = point.x + screen_size.0 / 2.0;
    let y = screen_size.1 - (point.y + screen_size.1 / 2.0);
    Vec2::new(x, y)
}

fn screen_bound(screen_size: Screen2, bound: BoundType) -> Vec2{
    let screen_hsize = (screen_size.0 / 2.0, screen_size.1 / 2.0);
    Collider::box_bound_offs(Vec2::from(screen_hsize), bound)
}

/// Helper functions

pub fn draw_actor(
    // assets: &mut Assets,
    ctx: &mut Context,
    actor: &Actor,
    screen_size: Screen2,
) -> GameResult {
    let circle = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        Vec2::new(0.0, 0.0),
        30.0,
        0.1,
        Color::WHITE,
    )?;
    let pos = world_to_screen_coords(screen_size, actor.pos);
    // let image = assets.actor_image(actor);
    let drawparams = graphics::DrawParam::new()
        .dest(pos);
    graphics::draw(ctx, &circle, drawparams)
}

pub fn draw_ground(
    ctx: &mut Context,
    width: f32,
    color: Color,
    screen_size: Screen2,
) -> GameResult {
    let line_center_y = GROUND_Y_COORD - width / 2.0;
    let points: Vec<Vec2> = [(-1000.0, line_center_y), (1000.0, line_center_y)]
        .into_iter()
        .map(|pos| world_to_screen_coords(screen_size, Vec2::new(pos.0, pos.1)))
        .collect();
    let line = graphics::Mesh::new_line(ctx, &points, width, color)?;
    let drawparams = graphics::DrawParam::new();
    graphics::draw(ctx, &line, drawparams)
}

/// Tests

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn box_collision_check_1() {
        let pos1 = Vec2::new(0.0, 0.0);
        let pos2 = Vec2::new(9.0, 9.0);
        let col1 = Collider::new_box(Vec2::new(5.0, 5.0));
        let col2 = Collider::new_box(Vec2::new(5.0, 5.0));
        let act1 = Actor::new(ActorType::Dino, pos1, Vec2::new(0.0,0.0), Vec2::new(0.0,0.0), col1);
        let act2 = Actor::new(ActorType::Dino, pos2, Vec2::new(0.0,0.0), Vec2::new(0.0,0.0), col2);
        assert_eq!(act1.check_collision(&act2), true);
    }

    #[test]
    fn box_collision_check_2() {
        let pos1 = Vec2::new(0.0, 0.0);
        let pos2 = Vec2::new(11.0, 9.0);
        let col1 = Collider::new_box(Vec2::new(5.0, 5.0));
        let col2 = Collider::new_box(Vec2::new(5.0, 5.0));
        let act1 = Actor::new(ActorType::Dino, pos1, Vec2::new(0.0,0.0), Vec2::new(0.0,0.0), col1);
        let act2 = Actor::new(ActorType::Dino, pos2, Vec2::new(0.0,0.0), Vec2::new(0.0,0.0), col2);
        assert_eq!(act1.check_collision(&act2), false);
    }

    // #[test]
    // fn out_of_screen_from_left(){
    //     let actor = Actor::new(
    //         ActorType::Dino,
    //         Vec2::new(-100.0, 0.0),
    //         Vec2::new(0.0, 0.0),
    //         Vec2::new(0.0, -700.0),
    //         Collider::BoxCollider(Vec2::new(30.0, 30.0)),
    //     );
    //     let screen_size = Vec2::new(100.0, 100.0);
    //     assert_eq!(
    //         false,
    //         actor.out_of_screen_from(screen_size, bound)
    //     )
    // }
}