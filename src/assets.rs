use crate::prelude::*;

pub struct Assets{
    pub dino_run_l: Sprite,
    pub dino_run_r: Sprite,
    pub dino_dead: Sprite,
    pub cactus1: Sprite,
    pub cactus2: Sprite,
    pub cactus3: Sprite,
    pub cactus4: Sprite,
    pub cloud: Sprite,
    pub ground1: Sprite,
    pub ground2: Sprite,
}

impl Assets{
    pub fn new(ctx: &mut Context) -> Assets {
        Assets{
            dino_run_l:     Sprite::new(ctx, "/dino_run_l.png"  ).unwrap(),
            dino_run_r:     Sprite::new(ctx, "/dino_run_r.png"  ).unwrap(),
            dino_dead:      Sprite::new(ctx, "/dino_dead.png"   ).unwrap(),
            cactus1:        Sprite::new(ctx, "/cactus_1.png"    ).unwrap(),
            cactus2:        Sprite::new(ctx, "/cactus_2.png"    ).unwrap(),
            cactus3:        Sprite::new(ctx, "/cactus_3.png"    ).unwrap(),
            cactus4:        Sprite::new(ctx, "/cactus_4.png"    ).unwrap(),
            cloud:          Sprite::new(ctx, "/cloud.png"       ).unwrap(),
            ground1:        Sprite::new(ctx, "/ground_1.png"    ).unwrap(),
            ground2:        Sprite::new(ctx, "/ground_2.png"    ).unwrap(),
        }
    }
}