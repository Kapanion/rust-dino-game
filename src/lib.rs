pub mod util;
pub mod types_and_constants;
pub mod ecs;
pub mod dino;
pub mod cactus;
pub mod input;
pub mod assets;

pub mod prelude{
    pub use types_and_constants::*;
    pub use util::*;
    pub use glam::*;

    pub use ggez::event::{self, KeyMods, KeyCode};
    pub use ggez::graphics::{self, Color, Image};
    pub use ggez::{timer, conf};
    pub use ggez::{Context, GameResult};

    pub use oorandom;

    pub use crate::*;
    pub use crate::ecs::*;
    pub use crate::ecs::{
        collision::BoxCollider,
        ezshape::CircleGraphic,
        movable::Movable,
        sprite::*,
        animation::Animation,
    };

    pub use crate::cactus::*;
    pub use crate::dino::*;
    pub use crate::input::*;
    pub use crate::assets::*;
}

use prelude::*;



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