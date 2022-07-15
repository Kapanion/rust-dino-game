use dino_game::prelude::*;

struct MainState {
    ecs: ECS,
    dino: usize,
    cactus: usize,
    screen_width: f32,
    screen_height: f32,
    // input: InputState,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        // DINO
        let mut ecs = ECS::new();

        // DINO
        let dino = ecs.new_entity();
        let dino_movable = Movable::new(
            v2!(-200.0, 200.0),
            v2!(0.0, 0.0),
            v2!(0.0, -100.0),
        );
        let dino_collider = BoxCollider::new(v2!(30.0, 30.0));

        ecs.add_component_to_entity(dino, dino_movable);
        ecs.add_component_to_entity(dino, dino_collider);
        ecs.add_component_to_entity(dino, CircleGraphic::new(30.0));
        
        // CACTUS
        let cactus = ecs.new_entity();
        ecs.add_component_to_entity(
            cactus,
            Movable::new(
                v2!(240.0, 0.0),
                v2!(-CACTUS_SPEED, 0.0),
                Vec2::ZERO,
            )
        );
        ecs.add_component_to_entity(cactus, CircleGraphic::new(40.0));

        let (width, height) = graphics::drawable_size(ctx);

        ecs.new_entity();

        let s = MainState{
            ecs,
            dino,
            cactus,
            screen_width: width,
            screen_height: height,
            // input: InputState::default(),
        };
        Ok(s)
    }
}


impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let dt = 1.0 / (DESIRED_FPS as f32);

            movable::update_pos(&mut self.ecs, self.dino, dt);
            movable::update_pos(&mut self.ecs, self.cactus, dt);
        }
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.8, 0.8, 0.8, 1.0].into());

        let screen_size = (self.screen_width, self.screen_height);

        draw_ground(ctx, 10.0, Color::BLACK, screen_size)?;

        for (circle_graphic, movable) in iter_zip!(self.ecs, CircleGraphic, Movable)
        {
            circle_graphic.draw(ctx, movable.pos, screen_size)?;
        }

        graphics::present(ctx)?;

        Ok(())
    }

    // fn key_down_event(
    //     &mut self,
    //     ctx: &mut Context,
    //     keycode: KeyCode,
    //     _keymod: KeyMods,
    //     _repeat: bool,
    // ) {
    //     match keycode{
    //         KeyCode::Space | KeyCode::Up => {
    //             self.input.jump_start();
    //         }
    //         _ => ()
    //     }
    // }

    // fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
    //     match keycode {
    //         KeyCode::Space | KeyCode::Up => {
    //             self.input.jump_end();
    //         }
    //         _ => (),
    //     }
    // }
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

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}