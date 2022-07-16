use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
struct CactusEntry{
    id: usize,
    active: bool,
}

impl CactusEntry{
    fn new(id: usize) -> CactusEntry{
        CactusEntry{
            id,
            active: false,
        }
    }
}

struct CactusPool{
    cacti: Vec<CactusEntry>,
    cur: usize,
}

impl CactusPool{
    fn new() -> CactusPool{
        CactusPool{
            cacti: Vec::new(),
            cur: 0,
        }
    }

    fn with_capacity(capacity: usize) -> CactusPool{
        CactusPool{
            cacti: Vec::with_capacity(capacity),
            cur: 0,
        }
    }

    fn add_cactus(&mut self, id: usize){
        self.cacti.push(CactusEntry::new(id));
    }

    fn activate_next(&mut self) -> Option<usize>{
        if self.cacti[self.cur].active {
            None
        }
        else{
            let res = self.cur;
            self.cur = (self.cur + 1) % self.cacti.len();
            self.cacti[res].active = true;
            Some(self.cacti[res].id)
        }
    }

    fn deactivate(&mut self, id: usize) {
        //TODO optimization
        for i in 0..self.cacti.len(){
            if self.cacti[i].id == id{
                self.cacti[i].active = false;
            }
        }
    }
}

pub struct CactusManager{
    pool: CactusPool,
    delay: f32,
    next_spawn_time: f32,
}

impl CactusManager{
    pub fn new(delay: f32) -> CactusManager{
        CactusManager{
            pool: CactusPool::new(),
            delay,
            next_spawn_time: 0.0,
        }
    }
    pub fn with_capacity(capacity: usize, delay: f32) -> CactusManager{
        CactusManager{
            pool: CactusPool::with_capacity(capacity),
            delay,
            next_spawn_time: 0.0,
        }
    }
    pub fn add_cactus(&mut self, id: usize){
        self.pool.add_cactus(id);
    }
    fn check_for_next_cactus(&mut self, ecs: &mut ECS, time: f32) {
        if self.next_spawn_time > time {return}
        let next_cactus = self.pool.activate_next().unwrap();
        let mut mov: Movable = ecs.get_component(next_cactus).unwrap();
        mov.pos.x = SCREEN.0 / 2.0 + 50.0;  //TODO more abstraction
        ecs.set_component::<Movable>(next_cactus, mov);
        self.next_spawn_time = time + self.delay;
    }
    pub fn update(&mut self, ecs: &mut ECS, time: f32, dt: f32){
        for i in 0..self.pool.cacti.len() {
            if self.pool.cacti[i].active{
                let pos_x = ecs.get_component::<Movable>(self.pool.cacti[i].id).unwrap().pos.x;
                if pos_x < - SCREEN.0 - 50.0 {
                    self.pool.deactivate(self.pool.cacti[i].id);
                }
                else{
                    Movable::update_pos(ecs, self.pool.cacti[i].id, dt);
                }
            }
        }
        self.check_for_next_cactus(ecs, time);
    }
    pub fn check_collision(&self, ecs: &ECS, entity_id: usize) -> bool{
        for i in 0..self.pool.cacti.len() {
            if self.pool.cacti[i].active{
                if BoxCollider::check_collision(ecs, entity_id, self.pool.cacti[i].id) {
                    return true;
                }
            }
        }
        false
    }
}