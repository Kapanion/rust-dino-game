use crate::prelude::*;

type Anim = Vec<Sprite>;

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
    pub fn new(ctx: &mut Context) -> Assets {
        let dino_run_l = Image::new(ctx, "/dino_run_l.png").unwrap();
        let dino_run_r = Image::new(ctx, "/dino_run_r.png").unwrap();
        let dino_anim_run = vec![
            Sprite::new(AssetTag::DinoRunL),
            Sprite::new(AssetTag::DinoRunR),
        ];
        let dino_anim_idle = vec![
            Sprite::new(AssetTag::DinoDead),
        ];
        let dino_anim_dead = vec![
            Sprite::new(AssetTag::DinoDead),
        ];
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
            return Some(anim[frame]);
        }
        None
    }
    pub fn get_anim_length(&self, tag: AssetTag) -> Option<usize> {
        if let Some(anim) = self.get_anim(tag) {
            return Some(anim.len());
        }
        None
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AssetTag{
    DinoRunL, DinoRunR, DinoDead,
    DinoAnimRun, DinoAnimJump, DinoAnimDead,
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