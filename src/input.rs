use crate::prelude::*;

#[derive(Debug, Default)]
pub struct InputState{
    jump: bool,
    pause: bool,
    game_active: bool,
    restart: bool,
}

impl InputState{
    pub fn new() -> InputState{
        InputState{
            jump: false,
            pause: false,
            game_active: true,
            restart: false,
        }
    }
    pub fn jump_start(&mut self){
        if self.pause {return}
        self.jump = true;
    }
    pub fn jump_end(&mut self){
        self.jump = false;
    }
    pub fn jump(&self) -> bool{
        self.jump
    }
    pub fn toggle_pause(&mut self){
        if !self.game_active {return}
        self.pause = !self.pause;
    }
    pub fn pause(&self) -> bool{
        self.pause
    }
    pub fn game_over(&mut self) {
        self.game_active = false;
    }
    pub fn game_active(&self) -> bool{
        self.game_active
    }
    pub fn restart(&self) -> bool{
        self.restart
    }
    pub fn set_restart(&mut self){
        self.restart = true;
    }
}

pub fn player_handle_input(ecs: &mut ECS, entity_id: usize, input: &mut InputState, _dt: f32) {
    if input.game_active{
        if input.jump() {
            ecs.get_component::<DinoController>(entity_id).unwrap().jump(ecs);
            input.jump_end();
        }
    }
    else{
        if input.jump() {
            input.set_restart();
        }
    }
}