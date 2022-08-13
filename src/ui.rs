use crate::prelude::*;

pub struct UIButton{
    pub pos: Vec2,
    active: bool,
    asset_tag: AssetTag,
    pub col: BoxCollider,
}

impl UIButton{
    pub fn new(assets: &Assets, image: AssetTag, pos: Vec2) -> Self{
        let img = assets.get_image(image).unwrap();
        Self{
            pos,
            active: true,
            asset_tag: image,
            col: BoxCollider::new(v2!(img.width() as f32 / 2., img.height() as f32 / 2.)),
        }
    }
    pub fn activate(&mut self){
        self.active = true;
    }
    pub fn deactivate(&mut self){
        self.active = false;
    }
}

impl Draw for UIButton{
    fn draw(&self, ctx: &mut Context, _ecs: &ECS, assets: &Assets, _entity_id: usize, _pos: Vec2, screen_size: Screen2) -> GameResult{
        if !self.active{
            return Ok(())
        }
        let img = assets.get_image(self.asset_tag).unwrap();
        let mut pos = world_to_screen_coords(screen_size, self.pos);
        // Floor the coordinates to prevent blurring
        pos.x = pos.x.floor();
        pos.y = pos.y.floor();
        let draw_params = graphics::DrawParam::new()
            .dest(pos - v2!(img.width() as f32 / 2., img.height() as f32 / 2.));
        graphics::draw(ctx, img, draw_params)
    }
}