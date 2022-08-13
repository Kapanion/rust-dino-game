// Inspired by Ian Kettlewell's blog post: https://ianjk.com/ecs-in-rust/

trait ComponentVec {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
}

pub struct ECS {
    entities_count: usize,
    component_vecs: Vec<Box<dyn ComponentVec>>,
}

impl ECS {
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

    pub fn add_component<ComponentType: 'static>(
        &mut self,
        entity: usize,
        component: ComponentType,
    ) {
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<Vec<Option<ComponentType>>>()
            {
                component_vec[entity] = Some(component);
                return;
            }
        }

        let mut new_component_vec: Vec<Option<ComponentType>> =
            Vec::with_capacity(self.entities_count);

        for _ in 0..self.entities_count {
            new_component_vec.push(None);
        }

        new_component_vec[entity] = Some(component);

        self.component_vecs
            .push(Box::new(new_component_vec));
    }

    pub fn get_component<ComponentType: 'static +  Copy + Clone>(
        &self,
        entity_id: usize,
    ) -> Option<ComponentType> {
        if let Some(component_vec) = self.borrow_component_vec::<ComponentType>(){
            return component_vec[entity_id];
        }
        None
    }

    pub fn set_component<ComponentType: 'static + Copy + Clone>(
        &mut self,
        entity_id: usize,
        new_component: ComponentType,
    ){
        if let Some(component_vec) = self.borrow_component_vec_mut::<ComponentType>(){
            component_vec[entity_id] = Some(new_component);
        }
    }

    pub fn borrow_component_vec<ComponentType: 'static>(
        &self,
    ) -> Option<&Vec<Option<ComponentType>>> {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<Vec<Option<ComponentType>>>()
            {
                return Some(component_vec);
            }
        }
        None
    }

    pub fn borrow_component_vec_mut<ComponentType: 'static>(
        &mut self,
    ) -> Option<&mut Vec<Option<ComponentType>>> {
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<Vec<Option<ComponentType>>>()
            {
                return Some(component_vec);
            }
        }
        None
    }

}

impl<T: 'static> ComponentVec for Vec<Option<T>> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        self.push(None)
    }
}