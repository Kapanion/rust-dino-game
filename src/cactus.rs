use crate::prelude::*;
use collision::BoundType;

#[derive(Clone, Copy, Debug, PartialEq)]
struct CactusEntry{
    id: usize,
    active: bool,
    additional_speed: f32,
}

impl CactusEntry{
    fn new(id: usize) -> CactusEntry{
        CactusEntry{
            id,
            active: false,
            additional_speed: 0.,
        }
    }
    fn set_additional_speed(&mut self, new_speed: f32){
        self.additional_speed = new_speed;
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

    fn add_ptero(&mut self, id: usize){
        let mut entry = CactusEntry::new(id);
        entry.set_additional_speed(PTERO_SPEED);
        self.cacti.push(entry);
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
        for i in 0..self.cacti.len(){
            if self.cacti[i].id == id{
                self.cacti[i].active = false;
            }
        }
    }

    fn deactivate_all(&mut self){
        for i in 0..self.cacti.len(){
            self.cacti[i].active = false;
        }
    }
}

pub struct CactusManager{
    pool: CactusPool,
    delay: f32,
    next_spawn_time: f32,
    rng: oorandom::Rand32,
    movable_ids: Box<Vec<usize>>,
    scroll_speed: f32,
}

impl CactusManager{
    pub fn new(delay: f32, movable_ids: Vec<usize>) -> CactusManager{
        CactusManager{
            pool: CactusPool::new(),
            delay,
            next_spawn_time: 0.0,
            rng: oorandom::Rand32::new(RNG_SEED),
            movable_ids: Box::new(movable_ids),
            scroll_speed: START_SCROLL_SPEED,
        }
    }
    pub fn with_capacity(capacity: usize, delay: f32, movable_ids: Vec<usize>) -> CactusManager{
        CactusManager{
            pool: CactusPool::with_capacity(capacity),
            delay,
            next_spawn_time: 0.0,
            rng: oorandom::Rand32::new(69420),
            movable_ids: Box::new(movable_ids),
            scroll_speed: START_SCROLL_SPEED,
        }
    }
    pub fn add_cactus(&mut self, id: usize){
        self.pool.add_cactus(id);
    }
    pub fn add_ptero(&mut self, id: usize){
        self.pool.add_ptero(id);
    }
    pub fn deactivate_all(&mut self){
        self.pool.deactivate_all();
    }
    pub fn restart(&mut self){
        self.deactivate_all();
        self.scroll_speed = START_SCROLL_SPEED;
    }
    fn check_for_next_cactus(&mut self, ecs: &mut ECS, time: f32) {
        if time < self.next_spawn_time {return}
        let next_cactus = self.pool.activate_next().unwrap();
        // println!("Cactus {next_cactus} activated");

        let mut mov: Movable = ecs.get_component(next_cactus).unwrap();
        mov.pos.x =
            SCREEN.0 / 2.0 - ecs.get_component::<Collider>(next_cactus)
                .unwrap()
                .get_bound_offset(BoundType::Left)
                .x;
        ecs.set_component::<Movable>(next_cactus, mov);

        self.update_movables_speed(ecs, self.scroll_speed);
        self.next_spawn_time = time + self.delay + self.rng.rand_float() * 1.3;
    }
    fn update_movables_speed(&self, ecs: &mut ECS, new_vel: f32){
        if self.scroll_speed >= MAX_SCROLL_SPEED {return}
        for id in self.movable_ids.iter() {
            let mut mov = ecs.get_component::<Movable>(*id).unwrap();
            mov.velocity.x = -new_vel;
            ecs.set_component(*id, mov);
        }
        for entry in self.pool.cacti.iter() {
            let id = entry.id;
            let mut mov = ecs.get_component::<Movable>(id).unwrap();
            mov.velocity.x = -(new_vel + entry.additional_speed);
            ecs.set_component(id, mov);
        }
    }
    fn update_scroll_speed(&mut self, dt: f32){
        if self.scroll_speed >= MAX_SCROLL_SPEED {return}
        self.scroll_speed += dt * 5.5;
    }
    pub fn update(&mut self, ecs: &mut ECS, time: f32, dt: f32){
        for i in 0..self.pool.cacti.len() {
            if self.pool.cacti[i].active{
                let id = self.pool.cacti[i].id;
                let col = ecs.get_component::<Collider>(id).unwrap();
                let right_bound = col.get_bound(ecs, id, BoundType::Right).x;
                if right_bound < - SCREEN.0 / 2.0 {
                    self.pool.deactivate(id);
                }
                else{
                    Movable::update_pos(ecs, id, dt);
                }
            }
        }
        self.check_for_next_cactus(ecs, time);
        self.update_scroll_speed(dt);
    }
    pub fn check_collision(&self, ecs: &ECS, entity_id: usize) -> bool{
        for i in 0..self.pool.cacti.len() {
            if self.pool.cacti[i].active{
                if Collider::check_entity_collision(ecs, entity_id, self.pool.cacti[i].id) {
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