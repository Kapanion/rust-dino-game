use ggez::{event, GameError, timer};
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use glam::*;
use std::env;
use std::path;
use dino_game::*;



struct MainState {
    dino: Actor,
    screen_width: f32,
    screen_height: f32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let dino = Actor::new(
            ActorType::Player,
            Vec2::new(0.0, 0.0),
            Vec2::new(20.0, 0.0),
            Vec2::new(0.0, 0.0),
            Collider::None,
        );
        let (width, height) = graphics::drawable_size(ctx);

        let s = MainState{
            dino,
            screen_width: width,
            screen_height: height,
        };
        Ok(s)
    }
}


impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let dt = 1.0 / (DESIRED_FPS as f32);

            self.dino.update_pos(dt);
        }
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.8, 0.8, 0.8, 1.0].into());

        let coords = (self.screen_width, self.screen_height);
        
        draw_actor(ctx, &self.dino, coords)?;

        graphics::present(ctx)?;

        Ok(())
    }
}


pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("test", "Kapanion").add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}