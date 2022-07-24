use dino_game::prelude::*;

struct MainState {
    ecs: ECS,
    dino: usize,
    ground1: usize,
    ground2: usize,
    cactus_manager: CactusManager,
    screen_width: f32,
    screen_height: f32,
    input: InputState,
    assets: Box<Assets>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let assets = Assets::new(ctx);

        let mut ecs = ECS::new();

        let ground1 = ecs.new_entity();
        let ground2 = ecs.new_entity();

        let cactus_tags = AssetTag::cactus_tags();
        let mut cactus_manager = CactusManager::with_capacity(cactus_tags.len(), 1.0);
        for _ in 0..cactus_tags.len() {
            let cactus = ecs.new_entity();
            cactus_manager.add_cactus(cactus);
        }

        let _ = ecs.new_entity();
        let dino = ecs.new_entity();

        let (width, height) = graphics::drawable_size(ctx);

        let s = MainState{
            ecs,
            dino,
            ground1,
            ground2,
            cactus_manager,
            screen_width: width,
            screen_height: height,
            input: InputState::new(),
            assets,
        };
        Ok(s)
    }
    fn start(&mut self, _ctx: &mut Context) {
        // DINO
        let mut dino_movable = Movable::new(
            v2!(-400.0, 0.0),
            v2!(0.0, 0.0),
            v2!(0.0, DINO_GRAVITY),
        );
        dino_movable.ground_check_on();
        let dino_collider_body = BoxCollider::new(v2!(14., 25.)).with_offset(v2!(-6., -18.));
        let dino_collider_head = BoxCollider::new(v2!(22., 17.)).with_offset(v2!(18., 32.));
        let dino_collider = Collider::new_double(dino_collider_body, dino_collider_head);
        let dino_anim = Animation::new(&mut self.assets, AssetTag::DinoAnimRun, 4);
        let dino_state_machine = AnimStateMachine::new(&mut self.assets, AssetTag::DinoStateMachine, DinoState::Run);

        self.ecs.add_component(self.dino, dino_movable);
        self.ecs.add_component(self.dino, dino_collider);
        self.ecs.add_component(self.dino, dino_anim);
        self.ecs.add_component(self.dino, DinoController::new(self.dino));
        self.ecs.add_component(self.dino, DinoState::Run);
        self.ecs.add_component(self.dino, dino_state_machine);
        // self.components.add_component(dino, CircleGraphic::new(47.0));

        // CACTUS
        let cactus_tags = AssetTag::cactus_tags();
        for i in 0..cactus_tags.len() {
            let cactus = self.cactus_manager.id(i);
            let img = self.assets.get_image(cactus_tags[i]).unwrap();
            let mut hs = v2!(img.width() as f32 / 2.0, img.height() as f32 / 2.0);
            let q: f32 = 0.7;
            let col_offs = v2!(0., -hs.y * (1. - q) / 2.);
            hs.y *= q;
            let col_low = BoxCollider::new(hs).with_offset(col_offs);
            let pad = 18.0 * img.height() as f32 / 100.0;
            let mut hs = v2!(img.width() as f32 / 2.0 - pad, img.height() as f32 / 2.0);
            hs.y -= 2.;
            let col_high = BoxCollider::new(hs);
            self.ecs.add_component(
                cactus,
                Movable::new(
                    v2!(SCREEN.0 + 50.0, GROUND_Y_COORD + img.height() as f32 / 2.0),
                    v2!(-SCROLL_SPEED, 0.0),
                    Vec2::ZERO,
                )
            );
            self.ecs.add_component(cactus,Collider::new_double(col_low, col_high));
            self.ecs.add_component(cactus, Sprite::new(cactus_tags[i]));
            // self.components.add_component(cactus, CircleGraphic::new(20.0));
        }

        // GROUND
        let mut ground_mov = Movable::new(
            v2!(0., 0.),
            v2!(-SCROLL_SPEED, 0.),
            v2!(0., 0.)
        );
        let ground_spr_1 = Sprite::new(AssetTag::Ground1);
        let ground_spr_2 = Sprite::new(AssetTag::Ground2);
        let w = self.assets.get_image(AssetTag::Ground1).unwrap().width() as f32;
        let ground_scr = EndlessScroll::new(w);

        self.ecs.add_component(self.ground1, ground_mov);
        self.ecs.add_component(self.ground1, ground_spr_1);
        self.ecs.add_component(self.ground1, ground_scr);

        ground_mov.pos.x += w;
        self.ecs.add_component(self.ground2, ground_mov);
        self.ecs.add_component(self.ground2, ground_spr_2);
        self.ecs.add_component(self.ground2, ground_scr);
    }
    fn restart(&mut self) {
        // DINO
        let mut dino_movable = Movable::new(
            v2!(-400.0, GROUND_Y_COORD),
            v2!(0.0, 0.0),
            v2!(0.0, DINO_GRAVITY),
        );
        dino_movable.ground_check_on();
        self.ecs.add_component(self.dino, dino_movable);
        self.ecs.set_component(self.dino, DinoState::Run);

        // CACTUS
        let cactus_tags = AssetTag::cactus_tags();
        for i in 0..cactus_tags.len() {
            let cactus = self.cactus_manager.id(i);
            let img = self.assets.get_image(cactus_tags[i]).unwrap();
            self.ecs.add_component(
                cactus,
                Movable::new(
                    v2!(SCREEN.0 + 50.0, GROUND_Y_COORD + img.height() as f32 / 2.0),
                    v2!(-SCROLL_SPEED, 0.0),
                    Vec2::ZERO,
                )
            );
        }

        self.cactus_manager.deactivate_all();

        self.input = InputState::new();
    }
}


impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let dt = 1.0 / (DESIRED_FPS as f32);
            let time = timer::time_since_start(ctx).as_secs_f32();

            player_handle_input(&mut self.ecs, self.dino, &mut self.input, dt);

            if self.input.restart() {
                self.restart();
                return Ok(());
            }
            if self.input.pause() || !self.input.game_active() {continue}

            update! {
                [&mut self.ecs, &self.assets, time, dt]
                Movable:                            self.dino, self.ground1, self.ground2;
                EndlessScroll:                      self.ground1, self.ground2;
                DinoController:                     self.dino;
                AnimStateMachine::<DinoState>:      self.dino;
                Animation:                          self.dino;
            };

            self.cactus_manager.update(&mut self.ecs, time, dt);

            // Losing the game
            if self.cactus_manager.check_collision(&mut self.ecs, self.dino) {
                println!("Game over!");
                self.ecs.set_component::<DinoState>(self.dino, DinoState::Dead);
                update! {
                    [&mut self.ecs, &self.assets, time, dt]
                    AnimStateMachine::<DinoState>:      self.dino;
                    Animation:                          self.dino;
                };
                self.input.game_over();
                self.draw(ctx)?;
                // let _ = event::quit(ctx);
            }
        }
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        const RGB_VAL: f32 = 247. / 255.;
        graphics::clear(ctx, Color::new(RGB_VAL, RGB_VAL, RGB_VAL, 1.0));

        let screen_size = (self.screen_width, self.screen_height);

        for (sprite, movable) in iter_zip!(self.ecs, Sprite, Movable) {
            sprite.draw(ctx, &self.ecs, &mut self.assets, 0, movable.pos, screen_size)?;
        }

        for (anim, movable) in iter_zip!(self.ecs, Animation, Movable) {
            anim.draw(ctx, &self.ecs, &mut self.assets, 0, movable.pos, screen_size)?;
        }

        // Draw colliders:
        // for (col, movable) in iter_zip!(self.ecs, Collider, Movable) {
        //     col.draw(ctx, &self.ecs, &mut self.assets, 0, movable.pos, screen_size)?;
        // }

        // Draw debug circles:
        // for (circle_graphic, movable) in iter_zip!(self.components, CircleGraphic, Movable) {
        //     circle_graphic.draw(ctx, movable.pos, screen_size)?;
        // }

        graphics::present(ctx)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
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
            KeyCode::Escape => {
                self.input.toggle_pause();
            }
            _ => (),
        }
    }
}


pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        std::path::PathBuf::from("./resources")
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