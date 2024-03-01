

use super::{
    component::Component,
    entity::EntityStorage,
    errors::{ArchetypeError, EntityAlreadyHaveComponent, EntityNotFounded},
    query::{Fetch, Query, QueryFetched},
};

pub struct Archetype {
    pub entities: Vec<EntityStorage>,
}

impl Archetype {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    pub fn spawn(&mut self, entity: EntityStorage) {
        self.entities.push(entity);
    }

    pub fn remove_entity(&mut self, entity_id: u32) {
        self.entities.retain(|e| e.id != entity_id);
    }

    pub fn remove_component_from_entity<T: Component>(&mut self, entity_id: u32) {
        if let Some(entity) = self.entities.iter_mut().find(|e| e.id == entity_id) {
            entity.remove_component::<T>();
        }
    }

    pub fn add_component_to_entity<T: Component + Send + Sync>(
        &mut self,
        entity_id: u32,
        component: T,
    ) -> Result<(), ArchetypeError> {
        if let Some(entity) = self.entities.iter_mut().find(|e| e.id == entity_id) {
            if let Ok(_) = entity.add_component(component) {
                return Ok(());
            }
            return Err(ArchetypeError::EntityAlreadyHaveComponentError(
                EntityAlreadyHaveComponent::new(entity_id, std::any::type_name::<T>().to_string()),
            ));
        }
        Err(ArchetypeError::EntityNotFoundedError(
            EntityNotFounded::new(entity_id),
        ))
    }

    pub fn get_entity(&self, entity_id: u32) -> Option<&EntityStorage> {
        self.entities.iter().find(|e| e.id == entity_id)
    }

    pub fn query_single<'a, T: Fetch<'a>>(&'a self) -> T::RawItem {
        <T>::fetch_single(self)
    }

    pub fn query<'a, T: Query<'a>>(&'a self) -> QueryFetched<T::Item> {
        <T>::get_components_in_all_entities(self)
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }
}
