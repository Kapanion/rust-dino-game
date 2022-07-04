// For testing
// Drawing circles

use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use glam::*;

use crate::util::*;
use super::*;

pub struct EzShape{
    radius: f32,
}

impl EzShape{
    pub fn new(radius: f32) -> EzShape{
        EzShape { radius }
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

impl Component for EzShape{
    fn start(&mut self) {
        // todo!()
    }

    fn update(&mut self, dt: f32) {
        // todo!()
    }
}