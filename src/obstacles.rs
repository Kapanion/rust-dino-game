use crate::prelude::*;
use collision::BoundType;

#[derive(Clone, Copy, Debug, PartialEq)]
struct ObstacleEntry {
    id: usize,
    active: bool,
    additional_speed: f32,
}

impl ObstacleEntry {
    fn new(id: usize) -> ObstacleEntry {
        ObstacleEntry {
            id,
            active: false,
            additional_speed: 0.,
        }
    }
    fn set_additional_speed(&mut self, new_speed: f32){
        self.additional_speed = new_speed;
    }
}

struct ObstaclePool {
    obstacles: Vec<ObstacleEntry>,
}

impl ObstaclePool {
    fn new() -> ObstaclePool {
        ObstaclePool {
            obstacles: Vec::new(),
        }
    }

    fn with_capacity(capacity: usize) -> ObstaclePool {
        ObstaclePool {
            obstacles: Vec::with_capacity(capacity),
        }
    }

    fn add_cactus(&mut self, id: usize){
        self.obstacles.push(ObstacleEntry::new(id));
    }

    fn add_ptero(&mut self, id: usize){
        let mut entry = ObstacleEntry::new(id);
        entry.set_additional_speed(PTERO_SPEED);
        self.obstacles.push(entry);
    }

    fn activate_next(&mut self, rng: &mut ThreadRng) -> Option<usize>{
        let next = (rng.gen::<u32>() as usize) % self.obstacles.len();
        for i in 0..self.obstacles.len(){
            let ind = (next + i) % self.obstacles.len();

            if !self.obstacles[ind].active {
                self.obstacles[ind].active = true;
                return Some(self.obstacles[ind].id);
            }
            
        }
        None
    }

    fn deactivate(&mut self, id: usize) {
        for i in 0..self.obstacles.len(){
            if self.obstacles[i].id == id{
                self.obstacles[i].active = false;
            }
        }
    }

    fn deactivate_all(&mut self){
        for i in 0..self.obstacles.len(){
            self.obstacles[i].active = false;
        }
    }
}

pub struct ObstacleManager {
    pool: ObstaclePool,
    delay: f32,
    next_spawn_time: f32,
    movable_ids: Box<Vec<usize>>,
    scroll_speed: f32,
}

impl ObstacleManager {
    pub fn new(delay: f32, movable_ids: Vec<usize>) -> ObstacleManager {
        ObstacleManager {
            pool: ObstaclePool::new(),
            delay,
            next_spawn_time: 0.0,
            movable_ids: Box::new(movable_ids),
            scroll_speed: START_SCROLL_SPEED,
        }
    }
    pub fn with_capacity(capacity: usize, delay: f32, movable_ids: Vec<usize>) -> ObstacleManager {
        ObstacleManager {
            pool: ObstaclePool::with_capacity(capacity),
            delay,
            next_spawn_time: 0.0,
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
    fn check_for_next_obstacle(&mut self, ecs: &mut ECS, rng: &mut ThreadRng, time: f32) {
        if time < self.next_spawn_time {return}
        let next_cactus = self.pool.activate_next(rng).unwrap();

        let mut mov: Movable = ecs.get_component(next_cactus).unwrap();
        mov.pos.x =
            SCREEN.0 / 2.0 - ecs.get_component::<Collider>(next_cactus)
                .unwrap()
                .get_bound_offset(BoundType::Left)
                .x;
        ecs.set_component::<Movable>(next_cactus, mov);

        self.update_movables_speed(ecs, self.scroll_speed);
        self.next_spawn_time = time + self.delay + rng.gen::<f32>() * 1.3;
    }
    fn update_movables_speed(&self, ecs: &mut ECS, new_vel: f32){
        if self.scroll_speed >= MAX_SCROLL_SPEED {return}
        for id in self.movable_ids.iter() {
            let mut mov = ecs.get_component::<Movable>(*id).unwrap();
            mov.velocity.x = -new_vel;
            ecs.set_component(*id, mov);
        }
        for entry in self.pool.obstacles.iter() {
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
    pub fn update(&mut self, ecs: &mut ECS, rng: &mut ThreadRng, time: f32, dt: f32){
        for i in 0..self.pool.obstacles.len() {
            if self.pool.obstacles[i].active{
                let id = self.pool.obstacles[i].id;
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
        self.check_for_next_obstacle(ecs, rng, time);
        self.update_scroll_speed(dt);
    }
    pub fn check_collision(&self, ecs: &ECS, entity_id: usize) -> bool {
        for i in 0..self.pool.obstacles.len() {
            if self.pool.obstacles[i].active {
                if Collider::check_entity_collision(ecs, entity_id, self.pool.obstacles[i].id) {
                    return true;
                }
            }
        }
        false
    }

    pub fn get_obstacle(&self, ecs: &ECS, entity_id: usize) -> (f64, f64) {
        let dino = Collider::get_pos(ecs,  entity_id);

        let mut y = 0.;
        let mut x = SCREEN.0 as f64;
        for i in 0..self.pool.obstacles.len() {
            if self.pool.obstacles[i].active {
                let obstacle = Collider::get_pos(ecs, self.pool.obstacles[i].id);
                if x > obstacle.x as f64 {
                    x = dino.distance(obstacle) as f64;
                    y = f64::from((GROUND_Y_COORD - obstacle.y) * -1.);
                } 
            }
        }
        
        return (x, y);
    }

    pub fn get_speed(&self) -> f64 {
        f64::from(self.scroll_speed)
    }

    pub fn ids(&self) -> Vec<usize>{
        let mut res = Vec::with_capacity(self.pool.obstacles.len());
        for entry in &self.pool.obstacles {
            res.push(entry.id);
        }
        res
    }
    pub fn id(&self, ind: usize) -> usize {
        self.pool.obstacles[ind].id
    }
}