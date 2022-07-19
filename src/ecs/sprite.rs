use crate::prelude::*;

pub trait Draw{
    fn draw(&self, ctx: &mut Context, assets: &Assets, pos: Vec2, screen_size: Screen2) -> GameResult;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sprite{
    asset_tag: AssetTag,
    offset: Vec2,
}

impl Sprite{
    pub fn new(asset_tag: AssetTag) -> Sprite {
        Sprite{
            asset_tag,
            offset: v2!(0.5, 0.5),
        }
    }
    pub fn set_offset(&mut self, offset: Vec2){
        self.offset = offset;
    }
    pub fn set_tag(&mut self, tag: AssetTag){
        self.asset_tag = tag;
    }
}

impl Draw for Sprite{
    fn draw(&self, ctx: &mut Context, assets: &Assets, pos: Vec2, screen_size: Screen2) -> GameResult{
        let pos = world_to_screen_coords(screen_size, pos);
        let drawparams = graphics::DrawParam::new()
            .dest(pos)
            .offset(self.offset);
        graphics::draw(ctx, assets.get_image(self.asset_tag).unwrap(), drawparams)
    }
}