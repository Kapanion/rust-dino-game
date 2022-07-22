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

/// Helper functions

pub fn draw_ground(
    ctx: &mut Context,
    width: f32,
    color: Color,
    screen_size: Screen2,
) -> GameResult {
    let line_center_y = GROUND_Y_COORD; // - width / 2.0;
    let points: Vec<Vec2> = [(-1000.0, line_center_y), (1000.0, line_center_y)]
        .into_iter()
        .map(|pos| world_to_screen_coords(screen_size, Vec2::new(pos.0, pos.1)))
        .collect();
    let line = graphics::Mesh::new_line(ctx, &points, width, color)?;
    let drawparams = graphics::DrawParam::new();
    graphics::draw(ctx, &line, drawparams)
}

/// World and screen positions

pub fn world_to_screen_coords(screen_size: Screen2, point: Vec2) -> Vec2 {
    let x = point.x + screen_size.0 / 2.0;
    let y = screen_size.1 - (point.y + screen_size.1 / 2.0);
    v2!(x, y)
}

// fn screen_bound(screen_size: Screen2, bound: BoundType) -> Vec2{
//     let screen_size_h = (screen_size.0 / 2.0, screen_size.1 / 2.0);
//     BoxCollider::box_bound_offs(Vec2::from(screen_size_h), bound)
// }