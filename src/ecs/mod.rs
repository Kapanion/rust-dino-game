// Components
// pub mod collision;
pub mod movable;
pub mod ezshape;

// ECS itself:

use std::cell::{RefCell, RefMut};

use crate::ecs::movable::Movable;


pub struct World {
    entities_count: usize,
    component_vecs: Vec<Box<dyn ComponentVec>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities_count: 0,
            component_vecs: Vec::new(),
        }
    }

    pub fn new_entity(&mut self) -> usize {
        let entity_id = self.entities_count;
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.push_none();
        }
        self.entities_count += 1;
        entity_id
    }

    pub fn add_component_to_entity<ComponentType: 'static + Component>(
        &mut self,
        entity: usize,
        component: ComponentType,
    ) {
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<Box<ComponentType>>>>>()
            {
                component_vec.get_mut()[entity] = Some(Box::new(component));
                return;
            }
        }

        let mut new_component_vec: Vec<Option<Box<ComponentType>>> =
            Vec::with_capacity(self.entities_count);

        for _ in 0..self.entities_count {
            new_component_vec.push(None);
        }

        new_component_vec[entity] = Some(Box::new(component));

        self.component_vecs
            .push(Box::new(RefCell::new(new_component_vec)));
    }

    pub fn borrow_component_vec<ComponentType: 'static + Component>(
        &self,
    ) -> Option<RefMut<Vec<Option<Box<ComponentType>>>>> {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<RefCell<Vec<Option<Box<ComponentType>>>>>()
            {
                return Some(component_vec.borrow_mut());
            }
        }
        None
    }
    // pub fn borrow_component_vec_1<ComponentType: 'static>(&self) -> Option<&Vec<Option<Box<ComponentType>>>> {
    //     for component_vec in self.component_vecs.iter() {
    //         if let Some(component_vec) = component_vec
    //             .as_any()
    //             .downcast_ref::<RefCell<Vec<Option<Box<ComponentType>>>>>()
    //         {
    //             return Some(component_vec.borrow_mut());
    //         }
    //     }
    //     None
    // }

    pub fn update_all(&self, dt: f32){
        for component_vec in self.component_vecs.iter() {
            component_vec.update_all(dt);
        }
    }
}

pub trait Component{
    fn start(&mut self);
    fn update(&mut self, dt: f32);
}

trait ComponentVec {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
    fn update_all(&self, dt: f32);
}

impl<T: 'static + Component> ComponentVec for RefCell<Vec<Option<Box<T>>>> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        self.get_mut().push(None)
    }

    fn update_all(&self, dt: f32) {
        for component in self.borrow_mut().iter_mut(){
            if let Some(component) = component{
                component.update(dt);
            }
        }
    }
}