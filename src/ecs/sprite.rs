use crate::prelude::*;

#[derive(Debug)]
pub struct Sprite{
    source: graphics::Image,
    offset: Vec2,
}

impl Sprite{
    pub fn new(ctx: &mut Context, path: &str) -> GameResult<Sprite>{
        let source = graphics::Image::new(ctx, path)?;
        Ok(Sprite{
            source,
            offset: v2!(0.5, 0.5),
        })
    }
    pub fn set_offset(&mut self, offset: Vec2){
        self.offset = offset;
    }
    pub fn draw(&self, ctx: &mut Context, pos: Vec2, screen_size: Screen2) -> GameResult{
        let pos = world_to_screen_coords(screen_size, pos);
        let drawparams = graphics::DrawParam::new()
            .dest(pos)
            .offset(self.offset);
        graphics::draw(ctx, &self.source, drawparams)
    }
}