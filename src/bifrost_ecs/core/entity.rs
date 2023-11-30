use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
    fmt::Debug,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use super::{
    component::{AsAny, Component, ComponentBundle},
    errors::{
        ComponentAlreadyBorrowed, ComponentAlreadyExists, ComponentError, ComponentNotFounded,
        ComponentUnableDowncast,
    },
};

pub type EntityId = u32;

impl<T: Component> AsAny for RwLock<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}


// impl<T: Debug + 'static> AsAny for T{
//     fn as_any(&self) -> &dyn std::any::Any {
//         self
//     }

//     fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
//         self
//     }
// }

#[derive(Debug)]
pub struct EntityStorage {
    pub id: EntityId,
    pub components: HashMap<TypeId, Box<dyn AsAny + Send + Sync>>,
}
pub trait FetchItem<'a> {
    type InnerItem;
    fn inner(&'a mut self) -> Self::InnerItem;
}

impl<'a, 'scene_borrow, T: 'a> FetchItem<'a> for RwLockReadGuard<'scene_borrow, T> {
    type InnerItem = &'a T;
    fn inner(&'a mut self) -> Self::InnerItem {
        self
    }
}

impl<'a, 'scene_borrow, T: 'a> FetchItem<'a> for RwLockWriteGuard<'scene_borrow, T> {
    type InnerItem = &'a mut T;
    fn inner(&'a mut self) -> Self::InnerItem {
        &mut *self
    }
}

impl EntityStorage {
    pub fn new(id: EntityId) -> Self {
        Self {
            id,
            components: HashMap::new(),
        }
    }

    pub fn get_component<'a, T: 'static + Debug>(&'a self) -> Result<&'a T, ComponentError> {
        let type_id = TypeId::of::<T>();
        if let Some(component_rw_ref) = self.components.get(&type_id) {
            if let Some(read) = component_rw_ref.as_any().downcast_ref::<RwLock<T>>() {
                if let Ok(mut c) = read.try_read() {
                    let component = c.inner();
                    let component = unsafe { std::mem::transmute::<&T, &'a T>(component) };
                    Ok(component)
                } else {
                    Err(ComponentError::ComponentAlreadyBorrowedError(
                        ComponentAlreadyBorrowed::new::<T>(),
                    ))
                }
            } else {
                Err(ComponentError::ComponentUnableDowncastError(
                    ComponentUnableDowncast::new::<T>(),
                ))
            }
        } else {
            Err(ComponentError::ComponentNotFoundedError(
                ComponentNotFounded::new::<T>(),
            ))
        }
    }

    pub fn get_component_mut<'a, T: 'static>(&'a self) -> Result<&'a mut T, ComponentError> {
        let type_id = TypeId::of::<T>();
        if let Some(component_rw_ref) = self.components.get(&type_id) {
            if let Some(write) = component_rw_ref.as_any().downcast_ref::<RwLock<T>>() {
                if let Ok(mut c) = write.try_write() {
                    let component = c.inner();
                    let component = unsafe { std::mem::transmute::<&mut T, &'a mut T>(component) };
                    Ok(component)
                } else {
                    Err(ComponentError::ComponentAlreadyBorrowedError(
                        ComponentAlreadyBorrowed::new::<T>(),
                    ))
                }
            } else {
                Err(ComponentError::ComponentUnableDowncastError(
                    ComponentUnableDowncast::new::<T>(),
                ))
            }
        } else {
            Err(ComponentError::ComponentNotFoundedError(
                ComponentNotFounded::new::<T>(),
            ))
        }
    }

    fn check_if_component_is_inside<T: Component>(&self) -> bool {
        self.components.iter().any(|it| *it.0 == TypeId::of::<T>())
    }

    pub fn add_component<T: Component>(&mut self, component: T) -> Result<(), ComponentError> {
        let type_id = TypeId::of::<T>();
        if self.check_if_component_is_inside::<T>() {
            return Err(ComponentError::ComponentAlreadyExistsError(
                ComponentAlreadyExists::new::<T>(),
            ));
        }
        self.components
            .insert(type_id, Box::new(RwLock::new(component)));
        Ok(())
    }

    pub fn add_components<T: ComponentBundle>(
        &mut self,
        unique_instances: &HashSet<TypeId>,
        components: T,
    ) {
        components.add_components_to_entity(unique_instances, self);
    }

    pub fn remove_component<T: Component>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.components.remove(&type_id);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(pub EntityId);