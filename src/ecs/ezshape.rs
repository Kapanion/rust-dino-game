// For testing
// Drawing circles

use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use glam::*;

use crate::util::*;
use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CircleGraphic {
    radius: f32,
}

impl CircleGraphic {
    pub fn new(radius: f32) -> CircleGraphic {
        CircleGraphic { radius }
    }

    pub fn draw(
        &self,
        ctx: &mut Context,
        pos: Vec2,
        screen_size: (f32, f32),
    ) -> GameResult
    {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            self.radius,
            0.1,
            Color::WHITE,
        )?;
        let pos = world_to_screen_coords(screen_size, pos);
        // let image = assets.actor_image(actor);
        let drawparams = graphics::DrawParam::new()
            .dest(pos);
        graphics::draw(ctx, &circle, drawparams)
    }
}
