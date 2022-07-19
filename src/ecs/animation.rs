use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Animation{
    asset_tag: AssetTag,
    fps: u8,
    len: usize,
    current_frame: usize,
    frame_time: f32,
    next_frame_upd: f32,
}

impl Animation {
    pub fn new(assets: &mut Assets, asset_tag: AssetTag, fps: u8) -> Animation {
        Animation{
            asset_tag,
            len: assets.get_anim_length(asset_tag).unwrap(),
            fps,
            current_frame: 0,
            frame_time: 1.0 / fps as f32,
            next_frame_upd: 0.0,
        }
    }
    pub fn update(&mut self, time: f32){
        if time < self.next_frame_upd {return}

        self.current_frame = (self.current_frame + 1) % self.len;

        self.next_frame_upd = time + self.frame_time;
    }
}

impl Draw for Animation{
    fn draw(&self, ctx: &mut Context, assets: &mut Assets, pos: Vec2, screen_size: Screen2) -> GameResult {
        assets.get_anim_frame(self.asset_tag, self.current_frame).unwrap().draw(ctx, assets, pos, screen_size)
    }
}