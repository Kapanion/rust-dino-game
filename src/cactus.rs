use std::ops::Bound;
use crate::collision::BoundType;
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
    rng: oorandom::Rand32,
}

impl CactusPool{
    fn new() -> CactusPool{
        CactusPool{
            cacti: Vec::new(),
            rng: oorandom::Rand32::new(RNG_SEED),
        }
    }

    fn with_capacity(capacity: usize) -> CactusPool{
        CactusPool{
            cacti: Vec::with_capacity(capacity),
            rng: oorandom::Rand32::new(RNG_SEED),
        }
    }

    fn add_cactus(&mut self, id: usize){
        self.cacti.push(CactusEntry::new(id));
    }

    fn activate_next(&mut self) -> Option<usize>{
        let next = (self.rng.rand_u32() as usize) % self.cacti.len();
        for i in 0..self.cacti.len(){
            let ind = (next + i) % self.cacti.len();
            if !self.cacti[ind].active {
                self.cacti[ind].active = true;
                return Some(self.cacti[ind].id);
            }
        }
        None
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
    rng: oorandom::Rand32,
}

impl CactusManager{
    pub fn new(delay: f32) -> CactusManager{
        CactusManager{
            pool: CactusPool::new(),
            delay,
            next_spawn_time: 0.0,
            rng: oorandom::Rand32::new(69420)
        }
    }
    pub fn with_capacity(capacity: usize, delay: f32) -> CactusManager{
        CactusManager{
            pool: CactusPool::with_capacity(capacity),
            delay,
            next_spawn_time: 0.0,
            rng: oorandom::Rand32::new(69420)
        }
    }
    pub fn add_cactus(&mut self, id: usize){
        self.pool.add_cactus(id);
    }
    fn check_for_next_cactus(&mut self, ecs: &mut ECS, time: f32) {
        if time < self.next_spawn_time {return}
        let next_cactus = self.pool.activate_next().unwrap();

        let mut mov: Movable = ecs.get_component(next_cactus).unwrap();
        mov.pos.x =
            SCREEN.0 / 2.0 - ecs.get_component::<BoxCollider>(next_cactus)
                .unwrap()
                .get_bound_offset(BoundType::Left)
                .x;
        ecs.set_component::<Movable>(next_cactus, mov);

        // let mut spr: Sprite = ecs.get_component(next_cactus).unwrap();
        // spr.set_tag(self.cactus_tags[self.rng.rand_u32() as usize % self.cactus_tags.len()]); // Random sprite
        // ecs.set_component(next_cactus, spr);

        self.next_spawn_time = time + self.delay;
    }
    pub fn update(&mut self, ecs: &mut ECS, time: f32, dt: f32){
        for i in 0..self.pool.cacti.len() {
            if self.pool.cacti[i].active{
                let id = self.pool.cacti[i].id;
                let col = ecs.get_component::<BoxCollider>(id).unwrap();
                if col.get_bound(ecs, id, BoundType::Right).x < - SCREEN.0 / 2.0 {
                    self.pool.deactivate(id);
                }
                else{
                    Movable::update_pos(ecs, id, dt);
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
    pub fn ids(&self) -> Vec<usize>{
        let mut res = Vec::with_capacity(self.pool.cacti.len());
        for entry in &self.pool.cacti{
            res.push(entry.id);
        }
        res
    }
    pub fn id(&self, ind: usize) -> usize {
        self.pool.cacti[ind].id
    }
}