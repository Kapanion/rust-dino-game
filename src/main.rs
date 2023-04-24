#![windows_subsystem = "windows"]

use dino_game::prelude::*;

use ggez::conf::Conf;
use ggez::event::MouseButton;
use std::time::Duration;

const MAX_ERROR: usize = 3;
const MAX_POPULATION: usize = 5000;

struct EntityIds {
    dino: usize,
    ground1: usize,
    ground2: usize,
    cloud: usize,
    ptero: usize,
    perceptron: usize,
}

struct Score {
    cur: f32,
    pub high: u32,
    next_sound: f32,
}

struct Dino {
    score: f32,
    step: usize,
    jump: usize,
    error: usize,
    actuator: f64,
    bias: f64,
    weights: Vec<f64>,
    perceptron: perceptron::Perceptron,
}

impl Dino {
    fn new(bias: f64, learning_rate: f64, weights: &Vec<f64>) -> Self {
        Dino {
            error: 0,
            score: 0.,
            step: 0,
            jump: 0,
            actuator: 0.,
            bias,
            weights: weights.clone(),
            perceptron: perceptron::Perceptron::new(bias, learning_rate, &weights),
        }
    }

    fn predict(&mut self, inputs: &Vec<f64>) -> f64 {
        self.actuator = self.perceptron.predict(&inputs);
        self.weights = self.get_weight().clone();

        if self.actuator < 0.5 {
            self.step += 1;
        } else {
            self.jump += 1;
        }

        self.actuator
    }

    fn adjust(&mut self, delta: f64, inputs: &Vec<f64>) {
        self.perceptron.error(delta, inputs);

        self.bias = self.perceptron.get_bias().clone();
        self.weights = self.perceptron.get_weight().clone();
    }

    fn score(&mut self, value: f32) {
        self.score += value;
    }

    fn error(&mut self, value: usize) {
        self.error += value;
    }

    fn get_weight(&mut self) -> Vec<f64> {
        self.perceptron.get_weight()
    }

    fn get_bias(&mut self) -> f64 {
        self.bias
    }
}

struct RNA {
    best: usize,
    learning_rate: f64,
    generation: usize,
    herd: Vec<Dino>,
}

impl RNA {
    pub fn new(bias: f64, learning_rate: f64) -> Self {
        // PESOS DE UM DINO TREINADO
        // let weights = vec![0.1870352501428702, -0.714498935466906, 0.680177527641077];

        let weights = Vec::new();

        let mut herd = Vec::new();

        for _ in 0..MAX_POPULATION {
            herd.push(Dino::new(bias, learning_rate, &weights))
        }

        RNA {
            best: 0,
            learning_rate,
            generation: 1,
            herd,
        }
    }

    fn restart(&mut self) {
        let mut herd = Vec::new();

        let (bias, weight) = self.get_dna();

        if weight.len() > 0 {
            println!(
                "DNA: 🧬\n\tw1:{}\n\tw2:{}\n\tw3:{}\n\tbias:{}",
                weight[0], weight[1], weight[2], bias
            );
        }

        for _ in 0..MAX_POPULATION {
            let value = if weight.len() == 0 {
                Vec::new()
            } else {
                vec![weight[0], weight[1], weight[2]]
            };
            herd.push(Dino::new(bias.clone(), self.learning_rate, &value));
        }

        while herd.len() < MAX_POPULATION as usize {
            herd.push(Dino::new(0.01, self.learning_rate, &Vec::new()))
        }

        self.generation += 1;
        self.herd = herd;
    }

    fn get_dna(&mut self) -> (f64, Vec<f64>) {
        let mut best = 0;
        let mut bias = 0.01;
        let mut weight: Vec<f64> = Vec::new();

        for (i, dino) in self.herd.iter().enumerate() {
            // MELHOR DNA: CORRE MAIS DO QUE PULA E PULA APENAS QUANDO NECESSÁRIO.
            if dino.score >= self.herd[best].score 
            && (dino.jump < dino.step) {
                best = i;
                bias = dino.bias.clone();
                weight = dino.weights.clone();
            }
        }

        if weight.len() > 0 {
            println!(
                "\nDINO: 🦖 \n\tscore:{}\n\tsteps:{}\n\tjumps:{}",
                self.herd[best].score as u32, self.herd[best].step, self.herd[best].jump
            );
        }

        return (bias, weight);
    }


    fn get_best(&mut self) -> f64 {
        if self.herd[self.best].error == MAX_ERROR {
            for (i, dino) in self.herd.iter().enumerate() {
                if dino.error < MAX_ERROR {
                    self.best = i;
                    break;
                }
            }
        }

        for (i, dino) in self.herd.iter().enumerate() {
            if (dino.error <= MAX_ERROR) 
            && (dino.score >= self.herd[self.best].score)
            && (dino.jump < dino.step) {
                self.best = i;
            }
        }

        self.herd[self.best].actuator
    }

    fn predict(&mut self, inputs: &Vec<f64>) -> f64 {
        for dino in self.herd.iter_mut() {
            if dino.error < MAX_ERROR {
                dino.predict(inputs);
            }
        }

        self.get_best()
    }

    fn adjust(&mut self, perceptron_inputs: &Vec<f64>, target: f64) {
        for dino in self.herd.iter_mut() {
            if dino.error < MAX_ERROR {
                if target == f64::from(0.0) && dino.actuator > f64::from(0.5) {
                    dino.error(1);
                    dino.adjust(f64::from(-1.0) * dino.actuator, perceptron_inputs);
                } else if target == f64::from(1.0) && dino.actuator < f64::from(0.5) {
                    dino.error(1);
                    dino.adjust(f64::from(1.0) - dino.actuator, perceptron_inputs);
                } else {
                    dino.score(1.0);
                }
            }
        }
    }

    fn get_generation(&self) -> usize {
        self.generation
    }

    fn get_population(&mut self) -> usize {
        self.herd
            .iter()
            .filter(|dino| dino.error < MAX_ERROR)
            .count()
    }

    fn get_actuator(&mut self) -> usize {
        if self.herd[self.best].actuator < 0.5 {
            0
        } else {
            1
        }
    }

    fn get_weights(&mut self) -> Vec<f64> {
        self.herd[self.best].get_weight()
    }

    fn get_bias(&mut self) -> f64 {
        self.herd[self.best].get_bias()
    }
}

struct MainState {
    ecs: ECS,
    ent: EntityIds,
    obstacle_manager: ObstacleManager,
    input: InputState,
    assets: Box<Assets>,
    rng: ThreadRng,
    restart_button: UIButton,
    pub score: Score,
    lose_time: f32,
    pub rna: RNA,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let assets = Assets::new(ctx);

        let mut ecs = ECS::new();

        let ground1 = ecs.new_entity();
        let ground2 = ecs.new_entity();

        let cloud = ecs.new_entity();

        let mov_vec = vec![ground1, ground2];

        let cactus_tags = AssetTag::cactus_tags();
        let mut obstacle_manager =
            ObstacleManager::with_capacity(cactus_tags.len(), CACTUS_MIN_DELAY, mov_vec);
        for _ in 0..cactus_tags.len() {
            let cactus = ecs.new_entity();
            obstacle_manager.add_cactus(cactus);
        }

        let ptero = ecs.new_entity();
        obstacle_manager.add_ptero(ptero);

        let dino = ecs.new_entity();

        let high_score = read_high_score_data(ctx);

        let mut restart_button = UIButton::new(&assets, AssetTag::RestartButton, v2!());
        restart_button.deactivate();

        let perceptron = ecs.new_entity();

        let rng = rand::thread_rng();

        let bias = 0.01;
        let learning_rate = 0.0005;

        let s = MainState {
            ecs,
            ent: EntityIds {
                dino,
                ground1,
                ground2,
                cloud,
                ptero,
                perceptron,
            },
            obstacle_manager,
            input: InputState::new(),
            assets,
            rng,
            restart_button,
            score: Score {
                cur: 0.,
                high: high_score,
                next_sound: 100.0,
            },
            lose_time: 0.,
            rna: RNA::new(bias, learning_rate),
        };
        Ok(s)
    }

    fn start(&mut self, _ctx: &mut Context) {
        // DINO
        let mut dino_movable = Movable::new(
            v2!(-400.0, GROUND_Y_COORD + 43.),
            v2!(0.0, 0.0),
            v2!(0.0, DINO_GRAVITY),
        );
        dino_movable.ground_check_on();
        let dino_collider_body = BoxCollider::new(v2!(14., 25.)).with_offset(v2!(-6., -18.));
        let dino_collider_head = BoxCollider::new(v2!(22., 17.)).with_offset(v2!(18., 32.));
        let dino_collider = Collider::new_double(dino_collider_body, dino_collider_head);
        let dino_anim = Animation::new(&mut self.assets, AssetTag::DinoAnimRun);
        let dino_state_machine =
            AnimStateMachine::new(&mut self.assets, AssetTag::DinoStateMachine, DinoState::Run);

        self.ecs.add_component(self.ent.dino, dino_movable);
        self.ecs.add_component(self.ent.dino, dino_collider);
        self.ecs.add_component(self.ent.dino, dino_anim);
        self.ecs.add_component(
            self.ent.dino,
            DinoController::new(self.ent.dino, AssetTag::JumpSound),
        );
        self.ecs.add_component(self.ent.dino, DinoState::Run);
        self.ecs.add_component(self.ent.dino, dino_state_machine);
        // self.components.add_component(dino, CircleGraphic::new(47.0));

        // PTERO
        let img = self.assets.get_image(AssetTag::Ptero1).unwrap();
        let ptero_wid = img.width() as f32;
        let ptero_col = Collider::new_single(
            BoxCollider::new(v2!(ptero_wid / 2. - 8., 20.)).with_offset(v2!(8., 4.)),
        );
        let ptero_scr = EndlessScroll::new(ptero_wid);
        let ptero_mov = Movable::new(
            v2!(SCREEN.0 + 50., GROUND_Y_COORD + 40.),
            v2!(-30., 0.),
            v2!(),
        );
        let ptero_anim = Animation::new(&self.assets, AssetTag::PteroAnim);

        self.ecs.add_component(self.ent.ptero, ptero_mov);
        self.ecs.add_component(self.ent.ptero, ptero_col);
        self.ecs.add_component(self.ent.ptero, ptero_anim);
        self.ecs.add_component(self.ent.ptero, ptero_scr);
        self.ecs.add_component(self.ent.ptero, Ptero::new());

        // CACTUS
        let cactus_tags = AssetTag::cactus_tags();
        for i in 0..cactus_tags.len() {
            let cactus = self.obstacle_manager.id(i);
            let img = self.assets.get_image(cactus_tags[i]).unwrap();
            // Some math for calculating cactus colliders
            let mut hs = v2!(img.width() as f32 / 2.0, img.height() as f32 / 2.0);
            let q: f32 = 0.7;
            let col_offs = v2!(0., -hs.y * (1. - q) / 2.);
            hs.y *= q;
            let col_low = BoxCollider::new(hs).with_offset(col_offs);
            let pad = 18.0 * img.height() as f32 / 100.0;
            let mut hs = v2!(img.width() as f32 / 2.0 - pad, img.height() as f32 / 2.0);
            hs.y -= 2.;
            let col_high = BoxCollider::new(hs);
            let offset_y = if img.height() == 100 {
                // big cactus
                if img.width() > 100 {
                    -2.
                } else {
                    -4.
                }
            } else {
                0.
            };
            self.ecs.add_component(
                cactus,
                Movable::new(
                    v2!(
                        SCREEN.0 + 50.0,
                        GROUND_Y_COORD + img.height() as f32 / 2.0 + offset_y
                    ),
                    v2!(-START_SCROLL_SPEED, 0.0),
                    Vec2::ZERO,
                ),
            );
            self.ecs
                .add_component(cactus, Collider::new_double(col_low, col_high));
            self.ecs.add_component(cactus, Sprite::new(cactus_tags[i]));
            // self.components.add_component(cactus, CircleGraphic::new(20.0));
        }

        // GROUND
        let mut ground_mov = Movable::new(v2!(0., -500.), v2!(-START_SCROLL_SPEED, 0.), v2!());
        let ground_spr_1 = Sprite::new(AssetTag::Ground1);
        let ground_spr_2 = Sprite::new(AssetTag::Ground2);
        let w = self.assets.get_image(AssetTag::Ground1).unwrap().width() as f32;
        let ground_scr = EndlessScroll::new(w);

        self.ecs.add_component(self.ent.ground1, ground_mov);
        self.ecs.add_component(self.ent.ground1, ground_spr_1);
        self.ecs.add_component(self.ent.ground1, ground_scr);

        ground_mov.pos.x += w + 0.;
        self.ecs.add_component(self.ent.ground2, ground_mov);
        self.ecs.add_component(self.ent.ground2, ground_spr_2);
        self.ecs.add_component(self.ent.ground2, ground_scr);

        // CLOUD
        let cloud_mov = Movable::new(
            v2!(0., -200.),
            v2!(-START_SCROLL_SPEED / 2.0, 0.),
            v2!(0., 0.),
        );
        let cloud_spr = Sprite::new(AssetTag::Cloud);
        let w = self.assets.get_image(AssetTag::Cloud).unwrap().width() as f32;
        let cloud_scr = EndlessScroll::new(w);

        self.ecs.add_component(self.ent.cloud, cloud_mov);
        self.ecs.add_component(self.ent.cloud, cloud_spr);
        self.ecs.add_component(self.ent.cloud, cloud_scr);

        // DRAW PERCEPTRON
        let perceptron_mov = Movable::new(v2!(0., 100.), v2!(0., 0.), v2!(0., 0.));
        let perceptron_spr = Sprite::new(AssetTag::Perceptron);
        let w = self.assets.get_image(AssetTag::Perceptron).unwrap().width() as f32;
        let perceptron_scr = EndlessScroll::new(w);

        self.ecs.add_component(self.ent.perceptron, perceptron_mov);
        self.ecs.add_component(self.ent.perceptron, perceptron_spr);
        self.ecs.add_component(self.ent.perceptron, perceptron_scr);
    }

    fn restart(&mut self, ctx: &mut Context) {
        self.input = InputState::new();

        if timer::time_since_start(ctx).as_secs_f32() < self.lose_time + 0.3 {
            self.input.game_over();
            return;
        }

        self.score.cur = 0.;

        // DINO
        let mut dino_movable = self.ecs.get_component::<Movable>(self.ent.dino).unwrap();
        dino_movable.pos.y = GROUND_Y_COORD + 43.;
        self.ecs.set_component(self.ent.dino, dino_movable);
        self.ecs.set_component(self.ent.dino, DinoState::Run);

        // CACTUS
        for id in self.obstacle_manager.ids() {
            let mut mov = self.ecs.get_component::<Movable>(id).unwrap();
            mov.pos.x = SCREEN.0 + 50.;
            self.ecs.set_component(id, mov);
        }
        self.obstacle_manager.restart();

        self.restart_button.deactivate();
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let dt = 1.0 / (DESIRED_FPS as f32);
            let time = timer::time_since_start(ctx).as_secs_f32();

            // INPUT STUFF
            input::player_handle_input(
                ctx,
                &mut self.ecs,
                &mut self.assets,
                self.ent.dino,
                &mut self.input,
                dt,
            );

            if self.input.restart() {
                self.restart(ctx);
                return Ok(());
            }
            if self.input.pause() || !self.input.game_active() {
                continue;
            }

            // EVERYTHING ELSE
            self.score.cur += dt * (10. + self.score.cur / 300.);
            if self.score.cur >= self.score.next_sound {
                let _ = self
                    .assets
                    .get_audio_mut(AssetTag::PointSound)
                    .unwrap()
                    .play(ctx);
                self.score.next_sound += 100.;
            }

            self.obstacle_manager
                .update(&mut self.ecs, &mut self.rng, time, dt);

            update! {
                [&mut self.ecs, &self.assets, &mut self.rng, time, dt]
                DinoController:                     self.ent.dino;
                EndlessScroll:                      self.ent.ground1, self.ent.ground2, self.ent.cloud;
                Movable:                            self.ent.dino, self.ent.ground1, self.ent.ground2, self.ent.cloud;
                Ptero:                              self.ent.ptero;
                AnimStateMachine::<DinoState>:      self.ent.dino;
                Animation:                          self.ent.dino, self.ent.ptero;
            };

            let (x, y) = self.obstacle_manager.get_obstacle(&mut self.ecs, self.ent.dino);
            let inputs = &vec![ self.obstacle_manager.get_speed(), x, y ];

            let result = self.rna.predict(&inputs);
            if result > 0.5 {
                if !self.input.jump() {
                    self.input.jump_start();
                } else {
                    self.input.jump_end();
                }
            }

            // Losing the game
            if self
                .obstacle_manager
                .check_collision(&mut self.ecs, self.ent.dino) || self.rna.get_population() == 0 {
                if self.rna.get_population() > 0 {
                    if self.rna.get_actuator() == 1 {
                        self.rna.adjust(&inputs, 0.0);
                    } else {
                        self.rna.adjust(&inputs, 1.0);
                    }
                } else {
                    self.rna.restart();
                }

                let _ = self
                    .assets
                    .get_audio_mut(AssetTag::DeathSound)
                    .unwrap()
                    .play(ctx);

                self.ecs
                    .set_component::<DinoState>(self.ent.dino, DinoState::Dead);
                update! {
                    [&mut self.ecs, &self.assets, &mut self.rng, time, dt]
                    AnimStateMachine::<DinoState>:      self.ent.dino;
                    Animation:                          self.ent.dino;
                };

                // HIGH SCORE
                let score = self.score.cur as u32;
                if self.score.high < score {
                    self.score.high = score;
                    write_high_score_data(ctx, score);
                }

                self.draw(ctx)?;
                self.input.game_over();
                self.input.set_restart();
            } else {
                if result > 0.5 {
                    self.rna.adjust(&inputs, 1.);
                } else {
                    self.rna.adjust(&inputs, 0.);
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if self.input.pause() || !self.input.game_active() {
            timer::sleep(Duration::new(0, 1_000_000_000 / DESIRED_FPS));
            return Ok(());
        }

        const RGB_VAL: f32 = 247. / 255.;
        graphics::clear(ctx, Color::new(RGB_VAL, RGB_VAL, RGB_VAL, 1.0));

        let screen_size = SCREEN;

        for (sprite, movable) in iter_zip!(self.ecs, Sprite, Movable) {
            sprite.draw(
                ctx,
                &self.ecs,
                &mut self.assets,
                0,
                movable.pos,
                screen_size,
            )?;
        }

        for (anim, movable) in iter_zip!(self.ecs, Animation, Movable) {
            anim.draw(
                ctx,
                &self.ecs,
                &mut self.assets,
                0,
                movable.pos,
                screen_size,
            )?;
        }

        // Draw colliders:
        if SHOW_COLLIDERS {
            for (col, movable) in iter_zip!(self.ecs, Collider, Movable) {
                if movable.pos.x == -400. {
                    col.draw(
                        ctx,
                        &self.ecs,
                        &mut self.assets,
                        0,
                        movable.pos,
                        screen_size,
                    )?;
                }
            }
        }

        const COL: f32 = 83. / 255.;

        // DRAWING TEXT:
        let score_str = format!(
            "Geração {:0>2} População {:0>5} Score: {:0>5}",
            self.rna.get_generation(),
            self.rna.get_population(),
            self.score.cur as u32
        );
        let score_display = graphics::Text::new((score_str, self.assets.font, 20.0));
        graphics::draw(
            ctx,
            &score_display,
            (v2!(15., 15.), 0.0, Color::new(COL, COL, COL, 1.0)),
        )?;

        // PERCEPTRON
        let s_str = format!("V: {:0.0}", self.obstacle_manager.get_speed() as u32);
        let s_display = graphics::Text::new((s_str, self.assets.font, 20.0));
        graphics::draw(
            ctx,
            &s_display,
            (
                v2!(SCREEN.0 - 1400., 300.),
                0.0,
                Color::new(COL, COL, COL, 1.0),
            ),
        )?;

        let (x, y) = self.obstacle_manager.get_obstacle(&mut self.ecs, self.ent.dino);
        let x_str = format!("X: {:0.0}", x);
        let x_display = graphics::Text::new((x_str, self.assets.font, 20.0));
        graphics::draw(
            ctx,
            &x_display,
            (
                v2!(SCREEN.0 - 1400., 490.),
                0.0,
                Color::new(COL, COL, COL, 1.0),
            ),
        )?;

        let y_str = format!("Y: {:0.0}", y);
        let y_display = graphics::Text::new((y_str, self.assets.font, 20.0));
        graphics::draw(
            ctx,
            &y_display,
            (
                v2!(SCREEN.0 - 1400., 680.),
                0.0,
                Color::new(COL, COL, COL, 1.0),
            ),
        )?;

        let w1_str = format!(
            "w1: {:0.4}",
            self.rna.get_weights().get(0).unwrap().clone() as f32
        );
        let w1_display = graphics::Text::new((w1_str, self.assets.font, 16.0));
        graphics::draw(
            ctx,
            &w1_display,
            (
                v2!(SCREEN.0 - 1100., 340.),
                0.0,
                Color::new(COL, COL, COL, 1.0),
            ),
        )?;

        let w2_str = format!(
            "w2: {:0.4}",
            self.rna.get_weights().get(1).unwrap().clone() as f32
        );
        let w2_display = graphics::Text::new((w2_str, self.assets.font, 16.0));
        graphics::draw(
            ctx,
            &w2_display,
            (
                v2!(SCREEN.0 - 1120., 510.),
                0.0,
                Color::new(COL, COL, COL, 1.0),
            ),
        )?;

        let w3_str = format!(
            "w3: {:0.4}",
            self.rna.get_weights().get(2).unwrap().clone() as f32
        );
        let w3_display = graphics::Text::new((w3_str, self.assets.font, 16.0));
        graphics::draw(
            ctx,
            &w3_display,
            (
                v2!(SCREEN.0 - 1100., 650.),
                0.0,
                Color::new(COL, COL, COL, 1.0),
            ),
        )?;

        let bias_str = format!("Bias: {:0.5}", self.rna.get_bias());
        let bias_display = graphics::Text::new((bias_str, self.assets.font, 16.0));
        graphics::draw(
            ctx,
            &bias_display,
            (
                v2!(SCREEN.0 - 880., 580.),
                0.0,
                Color::new(COL, COL, COL, 1.0),
            ),
        )?;

        let output_display = if self.rna.get_actuator() == 0 {
            graphics::Text::new(("CORRER", self.assets.font, 20.0))
        } else {
            graphics::Text::new(("PULAR", self.assets.font, 20.0))
        };
        graphics::draw(
            ctx,
            &output_display,
            (
                v2!(SCREEN.0 - 530., 490.),
                0.0,
                Color::new(COL, COL, COL, 1.0),
            ),
        )?;

        graphics::present(ctx)?;

        timer::yield_now();
        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        let world_pos = screen_to_world_coords(SCREEN, v2!(x, y));
        if button == MouseButton::Left {
            if self
                .restart_button
                .col
                .contains_point(self.restart_button.pos, world_pos)
            {
                self.restart(ctx);
            }
        }
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Space | KeyCode::Up => {
                self.input.jump_start();
            }
            _ => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Space | KeyCode::Up => {
                self.input.jump_end();
            }
            KeyCode::Q => {
                if PAUSE_ENABLED {
                    self.input.toggle_pause();
                }
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

    let (w, h) = SCREEN;

    let cb = ggez::ContextBuilder::new("rna-dino", "Kapanion, Eugenio Cunha")
        .default_conf(Conf::new())
        .window_setup(
            conf::WindowSetup::default()
                .icon("/images/dino_idle.png")
                .title("Como Treinar Seu Dinossauro"),
        )
        .window_mode(conf::WindowMode::default().dimensions(w, h))
        .add_resource_path(resource_dir);

    let (mut ctx, event_loop) = cb.build()?;

    let mut state = MainState::new(&mut ctx)?;
    state.start(&mut ctx);
    event::run(ctx, event_loop, state)
}
