// pub mod actor;
pub mod util;
pub mod types_and_constants;
// pub mod collider;
pub mod ecs;
pub mod cactus;
pub mod input;
pub mod assets;

pub mod prelude{
    // pub use actor::*;
    // pub use collider::*;
    pub use types_and_constants::*;
    pub use util::*;
    pub use glam::*;

    pub use ggez::event::{self, KeyMods, KeyCode};
    pub use ggez::graphics::{self, Color};
    pub use ggez::{timer, conf};
    pub use ggez::{Context, GameResult};

    pub use oorandom;

    pub use std::env;
    pub use std::path;
    pub use std::rc::Rc;

    pub use crate::*;
    pub use crate::ecs::*;
    pub use crate::ecs::{
        collision::BoxCollider,
        ezshape::CircleGraphic,
        movable::Movable,
        sprite::*,
    };

    pub use crate::cactus::*;
    pub use crate::input::*;
    pub use crate::assets::*;
}

use prelude::*;



/// Helper functions

// pub fn draw_actor(
//     // assets: &mut Assets,
//     ctx: &mut Context,
//     actor: &Actor,
//     screen_size: Screen2,
// ) -> GameResult {
//     let circle = graphics::Mesh::new_circle(
//         ctx,
//         graphics::DrawMode::fill(),
//         Vec2::new(0.0, 0.0),
//         30.0,
//         0.1,
//         Color::WHITE,
//     )?;
//     let pos = world_to_screen_coords(screen_size, actor.pos);
//     // let image = assets.actor_image(actor);
//     let drawparams = graphics::DrawParam::new()
//         .dest(pos);
//     graphics::draw(ctx, &circle, drawparams)
// }

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

/// Tests

#[cfg(test)]
mod tests {
    // use super::*;
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