use crate::prelude::*;

pub enum DinoState{
    Run, Jump, Dead,
}

// #[derive(Copy, Clone, Debug, PartialEq)]
// pub struct AnimStateMachine<State: Copy + Clone>{
//     pub anim_from_state: dyn FnOnce(State) -> Animation,
// }
//
// impl<State: Copy + Clone> AnimStateMachine<State>{
//     pub fn new(anim_from_state: Box<dyn FnOnce(State) -> Animation>) -> Box<AnimStateMachine<State>> {
//         Box::new(
//             AnimStateMachine{
//                 an im_from_state: *anim_from_state,
//             }
//         )
//     }
//     pub fn update(&self, ecs: &mut ECS, assets: &mut Assets, entity_id: usize) {
//         self.anim_from_state(ecs.get_component::<State>(entity_id)).update();
//     }
// }