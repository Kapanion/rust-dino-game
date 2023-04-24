use crate::prelude::*;

type Anim = (Vec<Sprite>, u8);      // (frames, fps)

pub struct Assets{
    pub dino_run_l:     Image,
    pub dino_run_r:     Image,
    pub dino_dead:      Image,
    pub dino_idle:      Image,
    pub cactus_small_1: Image,
    pub cactus_small_2: Image,
    pub cactus_small_3: Image,
    pub cactus_big_1:   Image,
    pub cactus_big_2:   Image,
    pub cactus_big_3:   Image,
    pub cloud:          Image,
    pub ground_1:       Image,
    pub ground_2:       Image,
    pub ptero_1:        Image,
    pub ptero_2:        Image,
    pub restart_button: Image,
    pub dino_anim_run:  Anim,
    pub dino_anim_idle: Anim,
    pub dino_anim_dead: Anim,
    pub ptero_anim:     Anim,
    pub font:           graphics::Font,
    pub jump_sound:     audio::Source,
    pub death_sound:    audio::Source,
    pub point_sound:    audio::Source,
    pub perceptron:     Image,
}

impl Assets{
    pub fn new(ctx: &mut Context) -> Box<Assets> {
        // DINO
        let dino_run_l = Image::new(ctx, "/images/dino_run_l.png").unwrap();
        let dino_run_r = Image::new(ctx, "/images/dino_run_r.png").unwrap();
        let dino_anim_run = (vec![
            Sprite::new(AssetTag::DinoRunL),
            Sprite::new(AssetTag::DinoRunR),
        ], 8);
        let dino_anim_idle = (vec![
            Sprite::new(AssetTag::DinoIdle),
        ], 1);
        let dino_anim_dead = (vec![
            Sprite::new(AssetTag::DinoDead),
        ], 1);

        // PTERO
        let ptero_1 = Image::new(ctx, "/images/ptero_1.png"  ).unwrap();
        let ptero_2 = Image::new(ctx, "/images/ptero_2.png"  ).unwrap();
        let ptero_anim = (vec![
            Sprite::new(AssetTag::Ptero1),
            Sprite::new(AssetTag::Ptero2),
        ], 4);

        // OTHER
        let font = graphics::Font::new(ctx, "/fonts/PressStart2P-Regular.ttf").unwrap();
        Box::new(
            Assets{
                dino_run_l,
                dino_run_r,
                dino_dead:      Image::new(ctx, "/images/dino_dead.png" ).unwrap(),
                dino_idle:      Image::new(ctx, "/images/dino_idle.png" ).unwrap(),
                cactus_small_1: Image::new(ctx, "/images/cactus_small_1.png"  ).unwrap(),
                cactus_small_2: Image::new(ctx, "/images/cactus_small_2.png"  ).unwrap(),
                cactus_small_3: Image::new(ctx, "/images/cactus_small_3.png"  ).unwrap(),
                cactus_big_1:   Image::new(ctx, "/images/cactus_big_1.png"  ).unwrap(),
                cactus_big_2:   Image::new(ctx, "/images/cactus_big_2.png"  ).unwrap(),
                cactus_big_3:   Image::new(ctx, "/images/cactus_big_3.png"  ).unwrap(),
                cloud:          Image::new(ctx, "/images/cloud.png"     ).unwrap(),
                ground_1:       Image::new(ctx, "/images/ground_1.png"  ).unwrap(),
                ground_2:       Image::new(ctx, "/images/ground_2.png"  ).unwrap(),
                ptero_1,
                ptero_2,
                restart_button: Image::new(ctx, "/images/restart_button.png"  ).unwrap(),
                dino_anim_run,
                dino_anim_idle,
                dino_anim_dead,
                ptero_anim,
                font,
                jump_sound:     audio::Source::new(ctx, "/sounds/jump.wav").unwrap(),
                death_sound:    audio::Source::new(ctx, "/sounds/death.wav").unwrap(),
                point_sound:    audio::Source::new(ctx, "/sounds/point.wav").unwrap(),
                perceptron:     Image::new(ctx, "/images/perceptron.png"  ).unwrap(),
            }
        )
    }
    pub fn get_image(&self, tag: AssetTag) -> Option<&Image> {
        match tag{
            AssetTag::DinoRunL      => Some(&self.dino_run_l),
            AssetTag::DinoRunR      => Some(&self.dino_run_r),
            AssetTag::DinoDead      => Some(&self.dino_dead),
            AssetTag::DinoIdle      => Some(&self.dino_idle),
            AssetTag::Ground1       => Some(&self.ground_1),
            AssetTag::Ground2       => Some(&self.ground_2),
            AssetTag::Cloud         => Some(&self.cloud),
            AssetTag::CactusSmall1  => Some(&self.cactus_small_1),
            AssetTag::CactusSmall2  => Some(&self.cactus_small_2),
            AssetTag::CactusSmall3  => Some(&self.cactus_small_3),
            AssetTag::CactusBig1    => Some(&self.cactus_big_1),
            AssetTag::CactusBig2    => Some(&self.cactus_big_2),
            AssetTag::CactusBig3    => Some(&self.cactus_big_3),
            AssetTag::Ptero1        => Some(&self.ptero_1),
            AssetTag::Ptero2        => Some(&self.ptero_2),
            AssetTag::RestartButton => Some(&self.restart_button),
            AssetTag::Perceptron => Some(&self.perceptron),
            _ => None
        }
    }
    pub fn get_audio(&self, tag: AssetTag) -> Option<&audio::Source> {
        match tag {
            AssetTag::JumpSound     => Some(&self.jump_sound),
            AssetTag::DeathSound    => Some(&self.death_sound),
            AssetTag::PointSound    => Some(&self.point_sound),
            _ => None
        }
    }
    pub fn get_audio_mut(&mut self, tag: AssetTag) -> Option<&mut audio::Source> {
        match tag {
            AssetTag::JumpSound     => Some(&mut self.jump_sound),
            AssetTag::DeathSound    => Some(&mut self.death_sound),
            AssetTag::PointSound    => Some(&mut self.point_sound),
            _ => None
        }
    }
    fn dino_state_machine(&self, state: DinoState) -> AssetTag{
        match state {
            DinoState::Run  => AssetTag::DinoAnimRun,
            DinoState::Jump => AssetTag::DinoAnimJump,
            DinoState::Dead => AssetTag::DinoAnimDead,
        }
    }
    pub fn get_anim(&self, tag: AssetTag) -> Option<&Anim> {
        match tag {
            AssetTag::DinoAnimRun   => Some(&self.dino_anim_run),
            AssetTag::DinoAnimJump  => Some(&self.dino_anim_idle),
            AssetTag::DinoAnimDead  => Some(&self.dino_anim_dead),
            AssetTag::PteroAnim     => Some(&self.ptero_anim),
            _ => None
        }
    }
    pub fn get_anim_frame(&self, tag: AssetTag, frame: usize) -> Option<Sprite> {
        if let Some(anim) = self.get_anim(tag) {
            return Some(anim.0[frame]);
        }
        None
    }
    pub fn get_anim_fps(&self, tag: AssetTag) -> Option<u8> {
        if let Some(anim) = self.get_anim(tag) {
            return Some(anim.1);
        }
        None
    }
    pub fn get_anim_length(&self, tag: AssetTag) -> Option<usize> {
        if let Some(anim) = self.get_anim(tag) {
            return Some(anim.0.len());
        }
        None
    }
    pub fn get_state_machine_anim<State: 'static>(&self, tag: AssetTag, state: State) -> Option<AssetTag> {
        match tag{
            AssetTag::DinoStateMachine => {
                if let Some(state) = (&state as &dyn std::any::Any).downcast_ref::<DinoState>(){
                    Some(self.dino_state_machine(*state))
                }
                else {None}
            }
            _ => None
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AssetTag{
    DinoRunL, DinoRunR, DinoDead, DinoIdle,
    DinoAnimRun, DinoAnimJump, DinoAnimDead,
    DinoStateMachine,
    CactusSmall1, CactusSmall2, CactusSmall3,
    CactusBig1, CactusBig2, CactusBig3,
    Ground1, Ground2,
    Ptero1, Ptero2,
    PteroAnim,
    Cloud,
    RestartButton,
    JumpSound, DeathSound, PointSound,
    Perceptron
}

impl AssetTag{
    pub fn cactus_tags() -> Vec<AssetTag> {
        vec![
            AssetTag::CactusSmall1,
            AssetTag::CactusSmall2,
            AssetTag::CactusSmall3,
            AssetTag::CactusBig1,
            AssetTag::CactusBig2,
            AssetTag::CactusBig3,
        ]
    }
}