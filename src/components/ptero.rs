use crate::collision::BoundType;
use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Ptero{
    needs_update: bool,
}

impl Ptero{
    pub fn new() -> Ptero{
        Ptero{
            needs_update: true,
        }
    }
}

impl Update for Ptero{
    fn update(ecs: &mut ECS, assets: &Assets, rng: &mut Rand32, entity_id: usize, time: f32, dt: f32) {
        let mut rightmost_x = ecs.get_component::<Collider>(entity_id).unwrap().get_bound(ecs, entity_id, BoundType::Right).x;
        let mut ptero = ecs.get_component::<Ptero>(entity_id).unwrap();
        if rightmost_x > - SCREEN.0 / 2.{
            if !ptero.needs_update {
                ptero.needs_update = true;
                ecs.set_component(entity_id, ptero);
            }
            return
        }
        if !ptero.needs_update {return}
        let mut mov = ecs.get_component::<Movable>(entity_id).unwrap();
        mov.pos.y = GROUND_Y_COORD + (rng.rand_u32() % 3) as f32 * 40. + 35.;
        ptero.needs_update = false;
        ecs.set_component(entity_id, ptero);
        ecs.set_component(entity_id, mov);
    }
}