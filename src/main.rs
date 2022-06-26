use ggez::event::{self, KeyMods, KeyCode};
use ggez::graphics::{self, Color};
use ggez::{timer, conf};
use ggez::{Context, GameResult};

use glam::*;

use std::env;
use std::path;

use dino_game::*;

use dino_game::core::collision::*;
struct MainState {
    dino: Actor,
    cactus: Actor,
    screen_width: f32,
    screen_height: f32,
    input: InputState,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let dino = Actor::new(
            ActorType::Dino,
            v2!(-200.0, 0.0),
            v2!(0.0, 0.0),
            v2!(0.0, -700.0),
            Collider::BoxCollider(v2!(30.0, 30.0)),
        );

        let cactus = Actor::new(
            ActorType::Cactus,
            v2!(240.0, 0.0),
            v2!(-CACTUS_SPEED, 0.0),
            Vec2::ZERO,
            Collider::BoxCollider(Vec2::new(30.0, 30.0)),
        );

        let (width, height) = graphics::drawable_size(ctx);

        let s = MainState{
            dino,
            cactus,
            screen_width: width,
            screen_height: height,
            input: InputState::default(),
        };
        Ok(s)
    }
}


impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let dt = 1.0 / (DESIRED_FPS as f32);

            player_handle_input(&mut self.dino, &mut self.input, dt);
            
            self.dino.update_pos(dt);
            self.cactus.update_pos(dt);
            self.cactus.check_respawn_right((self.screen_width, self.screen_height));

            if self.dino.check_collision(&self.cactus){
                event::quit(ctx);
            }
        }
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.8, 0.8, 0.8, 1.0].into());

        let screen_size = (self.screen_width, self.screen_height);

        draw_ground(ctx, 40.0, Color::BLACK, screen_size)?;

        draw_actor(ctx, &self.dino, screen_size)?;
        draw_actor(ctx, &self.cactus, screen_size)?;


        graphics::present(ctx)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode{
            KeyCode::Space | KeyCode::Up => {
                self.input.jump_start();
            }
            _ => ()
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Space | KeyCode::Up => {
                self.input.jump_end();
            }
            _ => (),
        }
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

    let cb = ggez::ContextBuilder::new("dino game", "Kapanion")
        .window_setup(conf::WindowSetup::default().title("Dino Game"))        
        .window_mode(conf::WindowMode::default().dimensions(640.0, 480.0))
        .add_resource_path(resource_dir);

    let (mut ctx, event_loop) = cb.build()?;

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}