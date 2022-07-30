pub mod macros;
pub mod types_and_constants;
pub mod components;
pub mod cactus;
pub mod input;
pub mod assets;
pub mod ecs;

pub mod prelude{
    pub use glam::*;

    pub use ggez::{
        conf,
        Context,
        event::{self, KeyCode, KeyMods}, GameResult,
        graphics::{self, Color, Image}, timer,
    };

    pub use oorandom;

    pub use crate::{
        *,
        assets::*,
        cactus::*,
        components::*,
        ecs::*,
        input::*,
        types_and_constants::*,
    };
}

use prelude::*;

pub trait Draw{
    fn draw(&self, ctx: &mut Context, ecs: &ECS, assets: &Assets, entity_id: usize, pos: Vec2, screen_size: Screen2) -> GameResult;
}

pub trait Update{
    fn update(ecs: &mut ECS, assets: &Assets, entity_id: usize, time: f32, dt: f32);
}

// World and screen positions
pub fn world_to_screen_coords(screen_size: Screen2, point: Vec2) -> Vec2 {
    let x = point.x + screen_size.0 / 2.0;
    let y = screen_size.1 - (point.y + screen_size.1 / 2.0);
    v2!(x, y)
}

// fn screen_bound(screen_size: Screen2, bound: BoundType) -> Vec2{
//     let screen_size_h = (screen_size.0 / 2.0, screen_size.1 / 2.0);
//     BoxCollider::box_bound_offs(Vec2::from(screen_size_h), bound)
// }