use crate::prelude::*;

pub struct Assets{
    pub dino_run_l: graphics::Image,
    pub dino_run_r: graphics::Image,
    pub dino_dead:  graphics::Image,
    pub cactus1:    graphics::Image,
    pub cactus2:    graphics::Image,
    pub cactus3:    graphics::Image,
    pub cactus4:    graphics::Image,
    pub cloud:      graphics::Image,
    pub ground1:    graphics::Image,
    pub ground2:    graphics::Image,
}

impl Assets{
    pub fn new(ctx: &mut Context) -> Assets {
        Assets{
            dino_run_l:     graphics::Image::new(ctx, "/dino_run_l.png").unwrap(),
            dino_run_r:     graphics::Image::new(ctx, "/dino_run_r.png").unwrap(),
            dino_dead:      graphics::Image::new(ctx, "/dino_dead.png" ).unwrap(),
            cactus1:        graphics::Image::new(ctx, "/cactus_1.png"  ).unwrap(),
            cactus2:        graphics::Image::new(ctx, "/cactus_2.png"  ).unwrap(),
            cactus3:        graphics::Image::new(ctx, "/cactus_3.png"  ).unwrap(),
            cactus4:        graphics::Image::new(ctx, "/cactus_4.png"  ).unwrap(),
            cloud:          graphics::Image::new(ctx, "/cloud.png"     ).unwrap(),
            ground1:        graphics::Image::new(ctx, "/ground_1.png"  ).unwrap(),
            ground2:        graphics::Image::new(ctx, "/ground_2.png"  ).unwrap(),
        }
    }
    pub fn get_image(&mut self, tag: AssetTag) -> &mut graphics::Image {
        match tag{
            AssetTag::DinoRunL  => &mut self.dino_run_l,
            AssetTag::DinoRunR  => &mut self.dino_run_r,
            AssetTag::DinoDead  => &mut self.dino_dead,
            AssetTag::Cactus1   => &mut self.cactus1,
            AssetTag::Cactus2   => &mut self.cactus2,
            AssetTag::Cactus3   => &mut self.cactus3,
            AssetTag::Cactus4   => &mut self.cactus4,
            AssetTag::Ground1   => &mut self.ground1,
            AssetTag::Ground2   => &mut self.ground2,
            AssetTag::Cloud     => &mut self.cloud,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AssetTag{
    DinoRunL, DinoRunR, DinoDead,
    Cactus1, Cactus2, Cactus3, Cactus4,
    Ground1, Ground2,
    Cloud,
}