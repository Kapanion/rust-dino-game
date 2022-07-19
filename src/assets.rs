use crate::prelude::*;

pub struct Assets{
    pub dino_run_l:     graphics::Image,
    pub dino_run_r:     graphics::Image,
    pub dino_dead:      graphics::Image,
    pub cactus_small_1: graphics::Image,
    pub cactus_small_2: graphics::Image,
    pub cactus_small_3: graphics::Image,
    pub cactus_big_1:   graphics::Image,
    pub cactus_big_2:   graphics::Image,
    pub cactus_big_3:   graphics::Image,
    pub cloud:          graphics::Image,
    pub ground1:        graphics::Image,
    pub ground2:        graphics::Image,
}

impl Assets{
    pub fn new(ctx: &mut Context) -> Assets {
        Assets{
            dino_run_l:     graphics::Image::new(ctx, "/dino_run_l.png").unwrap(),
            dino_run_r:     graphics::Image::new(ctx, "/dino_run_r.png").unwrap(),
            dino_dead:      graphics::Image::new(ctx, "/dino_dead.png" ).unwrap(),
            cactus_small_1: graphics::Image::new(ctx, "/cactus_small_1.png"  ).unwrap(),
            cactus_small_2: graphics::Image::new(ctx, "/cactus_small_2.png"  ).unwrap(),
            cactus_small_3: graphics::Image::new(ctx, "/cactus_small_3.png"  ).unwrap(),
            cactus_big_1:   graphics::Image::new(ctx, "/cactus_big_1.png"  ).unwrap(),
            cactus_big_2:   graphics::Image::new(ctx, "/cactus_big_2.png"  ).unwrap(),
            cactus_big_3:   graphics::Image::new(ctx, "/cactus_big_3.png"  ).unwrap(),
            cloud:          graphics::Image::new(ctx, "/cloud.png"     ).unwrap(),
            ground1:        graphics::Image::new(ctx, "/ground_1.png"  ).unwrap(),
            ground2:        graphics::Image::new(ctx, "/ground_2.png"  ).unwrap(),
        }
    }
    pub fn get_image(&mut self, tag: AssetTag) -> &mut graphics::Image {
        match tag{
            AssetTag::DinoRunL      => &mut self.dino_run_l,
            AssetTag::DinoRunR      => &mut self.dino_run_r,
            AssetTag::DinoDead      => &mut self.dino_dead,
            AssetTag::Ground1       => &mut self.ground1,
            AssetTag::Ground2       => &mut self.ground2,
            AssetTag::Cloud         => &mut self.cloud,
            AssetTag::CactusSmall1  => &mut self.cactus_small_1,
            AssetTag::CactusSmall2  => &mut self.cactus_small_2,
            AssetTag::CactusSmall3  => &mut self.cactus_small_3,
            AssetTag::CactusBig1    => &mut self.cactus_big_1,
            AssetTag::CactusBig2    => &mut self.cactus_big_2,
            AssetTag::CactusBig3    => &mut self.cactus_big_3,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AssetTag{
    DinoRunL, DinoRunR, DinoDead,
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