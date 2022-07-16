use ggez::GameError;
use dino_game::prelude::*;

struct MainState {
    ecs: ECS,
    dino: usize,
    cactus_manager: CactusManager,
    screen_width: f32,
    screen_height: f32,
    input: InputState,
    assets: Assets,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let assets = Assets::new(ctx);

        let mut ecs = ECS::new();
        let dino = ecs.new_entity();

        const POOL_SIZE: usize = 5;
        let mut cactus_manager = CactusManager::with_capacity(POOL_SIZE, 1.0);
        for _ in 0..POOL_SIZE {
            let cactus = ecs.new_entity();
            cactus_manager.add_cactus(cactus);
        }

        let (width, height) = graphics::drawable_size(ctx);

        let s = MainState{
            ecs,
            dino,
            cactus_manager,
            screen_width: width,
            screen_height: height,
            input: InputState::default(),
            assets,
        };
        Ok(s)
    }
    fn start(&mut self, ctx: &mut Context) {
        let dino_movable = Movable::new(
            v2!(-200.0, 200.0),
            v2!(0.0, 0.0),
            v2!(0.0, DINO_GRAVITY),
        );
        let mut dino_collider = BoxCollider::new(v2!(20., 47.));
        dino_collider.ground_check_on();

        self.ecs.add_component(self.dino, dino_movable);
        self.ecs.add_component(self.dino, dino_collider);
        self.ecs.add_component(self.dino, Sprite::new(ctx, "/dino_run_l.png").unwrap());
        // self.ecs.add_component(dino, CircleGraphic::new(47.0));

        for cactus in self.cactus_manager.ids() {
            let hs = v2!(51.0, 35.0);
            self.ecs.add_component(
                cactus,
                Movable::new(
                    v2!(SCREEN.0 + 50.0, GROUND_Y_COORD + hs.y),
                    v2!(-CACTUS_SPEED, 0.0),
                    Vec2::ZERO,
                )
            );
            self.ecs.add_component(cactus,BoxCollider::new(hs));
            // self.ecs.add_component(cactus, CircleGraphic::new(20.0));
            self.ecs.add_component(cactus, Sprite::new(ctx, "/cactus_1.png").unwrap());
        }
    }
}


impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let dt = 1.0 / (DESIRED_FPS as f32);
            let time = timer::time_since_start(ctx).as_secs_f32();

            player_handle_input(&mut self.ecs, self.dino, &mut self.input, dt);

            Movable::update_pos(&mut self.ecs, self.dino, dt);
            self.cactus_manager.update(&mut self.ecs, time, dt);

            if self.cactus_manager.check_collision(&mut self.ecs, self.dino) {
                println!("Game over!");
                let _ = event::quit(ctx);
            }
        }
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.8, 0.8, 0.8, 1.0].into());

        let screen_size = (self.screen_width, self.screen_height);

        draw_ground(ctx, 10.0, Color::BLACK, screen_size)?;

        for (sprite, movable) in iter_zip!(self.ecs, Sprite, Movable) {
            sprite.draw(ctx, movable.pos, screen_size)?;
        }

        // for (circle_graphic, movable) in iter_zip!(self.ecs, CircleGraphic, Movable) {
        //     circle_graphic.draw(ctx, movable.pos, screen_size)?;
        // }

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

    let (w,h) = SCREEN;

    let cb = ggez::ContextBuilder::new("dino game", "Kapanion")
        .window_setup(conf::WindowSetup::default().title("Dino Game"))        
        .window_mode(conf::WindowMode::default().dimensions(w, h))
        .add_resource_path(resource_dir);

    let (mut ctx, event_loop) = cb.build()?;

    let mut state = MainState::new(&mut ctx)?;
    state.start(&mut ctx);
    event::run(ctx, event_loop, state)
}