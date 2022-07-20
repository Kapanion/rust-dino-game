use crate::prelude::*;

use std::any::TypeId;

type Anim = (Vec<Sprite>, u8);      // (frames, fps)

pub struct Assets{
    pub dino_run_l:     Image,
    pub dino_run_r:     Image,
    pub dino_dead:      Image,
    pub cactus_small_1: Image,
    pub cactus_small_2: Image,
    pub cactus_small_3: Image,
    pub cactus_big_1:   Image,
    pub cactus_big_2:   Image,
    pub cactus_big_3:   Image,
    pub cloud:          Image,
    pub ground1:        Image,
    pub ground2:        Image,
    pub dino_anim_run:  Anim,
    pub dino_anim_idle: Anim,
    pub dino_anim_dead: Anim,
}

impl Assets{
    pub fn new(ctx: &mut Context) -> Box<Assets> {
        let dino_run_l = Image::new(ctx, "/dino_run_l.png").unwrap();
        let dino_run_r = Image::new(ctx, "/dino_run_r.png").unwrap();
        let dino_anim_run = (vec![
            Sprite::new(AssetTag::DinoRunL),
            Sprite::new(AssetTag::DinoRunR),
        ], 4);
        let dino_anim_idle = (vec![
            Sprite::new(AssetTag::DinoDead),
        ], 1);
        let dino_anim_dead = (vec![
            Sprite::new(AssetTag::DinoDead),
        ], 1);
        Box::new(
            Assets{
                dino_run_l,
                dino_run_r,
                dino_dead:      Image::new(ctx, "/dino_dead.png" ).unwrap(),
                cactus_small_1: Image::new(ctx, "/cactus_small_1.png"  ).unwrap(),
                cactus_small_2: Image::new(ctx, "/cactus_small_2.png"  ).unwrap(),
                cactus_small_3: Image::new(ctx, "/cactus_small_3.png"  ).unwrap(),
                cactus_big_1:   Image::new(ctx, "/cactus_big_1.png"  ).unwrap(),
                cactus_big_2:   Image::new(ctx, "/cactus_big_2.png"  ).unwrap(),
                cactus_big_3:   Image::new(ctx, "/cactus_big_3.png"  ).unwrap(),
                cloud:          Image::new(ctx, "/cloud.png"     ).unwrap(),
                ground1:        Image::new(ctx, "/ground_1.png"  ).unwrap(),
                ground2:        Image::new(ctx, "/ground_2.png"  ).unwrap(),
                dino_anim_run,
                dino_anim_idle,
                dino_anim_dead,
            }
        )
    }
    pub fn get_image(&self, tag: AssetTag) -> Option<&Image> {
        match tag{
            AssetTag::DinoRunL      => Some(&self.dino_run_l),
            AssetTag::DinoRunR      => Some(&self.dino_run_r),
            AssetTag::DinoDead      => Some(&self.dino_dead),
            AssetTag::Ground1       => Some(&self.ground1),
            AssetTag::Ground2       => Some(&self.ground2),
            AssetTag::Cloud         => Some(&self.cloud),
            AssetTag::CactusSmall1  => Some(&self.cactus_small_1),
            AssetTag::CactusSmall2  => Some(&self.cactus_small_2),
            AssetTag::CactusSmall3  => Some(&self.cactus_small_3),
            AssetTag::CactusBig1    => Some(&self.cactus_big_1),
            AssetTag::CactusBig2    => Some(&self.cactus_big_2),
            AssetTag::CactusBig3    => Some(&self.cactus_big_3),
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
                // if TypeId::of::<State>() == TypeId::of::<DinoState>(){
                //     Some(self.dino_state_machine(state))
                // }
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
    DinoRunL, DinoRunR, DinoDead,
    DinoAnimRun, DinoAnimJump, DinoAnimDead,
    DinoStateMachine,
    CactusSmall1, CactusSmall2, CactusSmall3,
    CactusBig1, CactusBig2, CactusBig3,
    Ground1, Ground2,
    Cloud,
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