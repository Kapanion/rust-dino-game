use crate::prelude::*;

#[derive(Debug, Default)]
pub struct InputState{
    jump: bool,
}

impl InputState{
    pub fn jump_start(&mut self){
        self.jump = true;
    }
    pub fn jump_end(&mut self){
        self.jump = false;
    }
    pub fn jump(&self) -> bool{
        self.jump
    }
}

pub fn player_handle_input(ecs: &mut ECS, entity_id: usize, input: &mut InputState, _dt: f32) {
    if input.jump() {
        ecs.get_component::<DinoController>(entity_id).unwrap().jump(ecs);
        input.jump_end();
    }
}