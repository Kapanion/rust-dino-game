use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DinoState{
    Run, Jump, Dead,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DinoController {
    entity_id: usize,
}

impl DinoController {
    pub fn new(entity_id: usize) -> DinoController {
        DinoController {
            entity_id,
        }
    }
    pub fn jump(&self, ecs: &mut ECS){
        let mut mov: Movable = ecs.get_component(self.entity_id).unwrap();
        mov.jump(JUMP_VELOCITY);
        ecs.set_component(self.entity_id, mov);
    }
}

impl Update for DinoController{
    fn update(ecs: &mut ECS, assets: &Assets, entity_id: usize, time: f32, dt: f32) {
        let mov: Movable = ecs.get_component(entity_id).unwrap();
        ecs.set_component::<DinoState>(entity_id, if mov.on_ground {DinoState::Run} else {DinoState::Jump});
    }
}