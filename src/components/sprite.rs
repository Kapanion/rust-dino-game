use crate::prelude::*;

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
    fn draw(&self, ctx: &mut Context, ecs: &ECS, assets: &Assets, entity_id: usize, pos: Vec2, screen_size: Screen2) -> GameResult{
        let mut pos = world_to_screen_coords(screen_size, pos);
        // Floor the coordinates to prevent blurring
        pos.x = pos.x.floor();
        pos.y = pos.y.floor();
        let draw_params = graphics::DrawParam::new()
            .dest(pos)
            .offset(self.offset);
        graphics::draw(ctx, assets.get_image(self.asset_tag).unwrap(), draw_params)
    }
}