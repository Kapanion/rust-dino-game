use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DinoState{
    Run, Jump, Dead,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DinoController {
    entity_id: usize,
    jump_sound_tag: AssetTag,
}

impl DinoController {
    pub fn new(entity_id: usize, jump_sound_tag: AssetTag) -> DinoController {
        DinoController {
            entity_id,
            jump_sound_tag,
        }
    }
    pub fn jump(&self, ctx: &Context, ecs: &mut ECS, assets: &mut Assets){
        let mut mov: Movable = ecs.get_component(self.entity_id).unwrap();
        let jump_success = mov.jump(JUMP_VELOCITY);
        if jump_success {
            ecs.set_component(self.entity_id, mov);
            let _ = assets.get_audio_mut(self.jump_sound_tag).unwrap().play(ctx);
        }
    }
}

impl Update for DinoController{
    fn update(ecs: &mut ECS, _assets: &Assets, entity_id: usize, _time: f32, _dt: f32) {
        let mov: Movable = ecs.get_component(entity_id).unwrap();
        ecs.set_component::<DinoState>(entity_id, if mov.on_ground {DinoState::Run} else {DinoState::Jump});
    }
}