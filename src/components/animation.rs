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
    pub fn new(assets: &Assets, asset_tag: AssetTag, fps: u8) -> Animation {
        Animation{
            asset_tag,
            len: assets.get_anim_length(asset_tag).unwrap(),
            fps,
            current_frame: 0,
            frame_time: 1.0 / fps as f32,
            next_frame_upd: 0.0,
        }
    }
    pub fn update_frame(&mut self, time: f32){
        if time < self.next_frame_upd {return}
        self.current_frame = (self.current_frame + 1) % self.len;
        self.next_frame_upd = time + self.frame_time;
    }
}

impl Update for Animation{
    fn update(ecs: &mut ECS, _assets: &Assets, entity_id: usize, time: f32, _dt: f32) {
        let mut anim = ecs.get_component::<Animation>(entity_id).unwrap();
        anim.update_frame(time);
        ecs.set_component(entity_id, anim);
    }
}

impl Draw for Animation{
    fn draw(&self, ctx: &mut Context, ecs: &ECS, assets: &Assets, entity_id: usize, pos: Vec2, screen_size: Screen2) -> GameResult {
        assets.get_anim_frame(self.asset_tag, self.current_frame).unwrap().draw(ctx, ecs, assets, entity_id, pos, screen_size)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AnimStateMachine<State: 'static + Copy + Clone + PartialEq>{
    asset_tag: AssetTag,
    current_state: State,
    current_anim_tag: AssetTag,
    current_anim: Animation,
}

impl<State: 'static + Copy + Clone + PartialEq> AnimStateMachine<State> {
    pub fn new(assets: &Assets, asset_tag: AssetTag, start_state: State) -> AnimStateMachine<State> {
        let current_anim_tag = assets.get_state_machine_anim(asset_tag, start_state).unwrap();
        let fps = assets.get_anim_fps(current_anim_tag).unwrap();
        AnimStateMachine {
            asset_tag,
            current_state: start_state,
            current_anim_tag,
            current_anim: Animation::new(assets, current_anim_tag, fps),
        }
    }
    pub fn update_state(&mut self, ecs: &mut ECS, assets: &Assets, entity_id: usize) {
        let new_state = ecs.get_component::<State>(entity_id);//.unwrap();
        if new_state == None { return }
        let new_state = new_state.unwrap();
        if self.current_state == new_state { return }
        self.current_state = new_state;
        let anim_tag = assets.get_state_machine_anim(self.asset_tag, new_state).unwrap();
        let anim_fps = assets.get_anim_fps(anim_tag).unwrap();
        ecs.set_component::<Animation>(entity_id, Animation::new(assets, anim_tag, anim_fps));
    }
}

impl<State: 'static + Copy + Clone + PartialEq> Update for AnimStateMachine<State>{
    fn update(ecs: &mut ECS, assets: &Assets, entity_id: usize, _time: f32, _dt: f32) {
        let mut anim = ecs.get_component::<AnimStateMachine<State>>(entity_id).unwrap();
        anim.update_state(ecs, assets, entity_id);
        ecs.set_component(entity_id, anim);
    }
}