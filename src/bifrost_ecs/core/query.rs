use std::ops::{Deref, DerefMut};

use super::{
    component::Component,
    entity::{Entity, EntityId},
    errors::ComponentError,
    scene::Scene,
};

pub struct QueryFetched<T> {
    result: QueryResult<T>,
}

impl<T> Deref for QueryFetched<T> {
    type Target = QueryResult<T>;

    fn deref(&self) -> &Self::Target {
        &self.result
    }
}

impl<T> DerefMut for QueryFetched<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.result
    }
}

pub trait FetchRaw<'a> {
    type RawItem;
    fn fetch(entity: &'a Entity) -> Result<Self::RawItem, ComponentError>;
    fn founded(entity: &'a Entity) -> bool {
        Self::fetch(entity).is_ok()
    }
}

pub trait Fetch<'a> {
    type RawItem;
    type Item: IntoIterator;
    fn fetch_single(scene: &'a Scene) -> Self::RawItem;
    fn fetch(scene: &'a Scene) -> Self::Item;
}

pub trait Query<'a> {
    type Item;
    fn get_components_in_all_entities(scene: &'a Scene) -> QueryFetched<Self::Item>;
}

impl<'a, T: Component> Fetch<'a> for &T {
    type RawItem = &'a T;
    type Item = Vec<(EntityId, Self::RawItem)>;

    fn fetch(scene: &'a Scene) -> Self::Item {
        scene
            .entities
            .iter()
            .filter_map(|entity| entity.get_component::<T>().ok().map(|it| (entity.id, it)))
            .collect()
    }

    fn fetch_single(scene: &'a Scene) -> Self::RawItem {
        let a = scene
            .entities
            .iter()
            .find_map(|entity| {
                entity
                    .get_component::<T>()
                    .ok()
                    .map(|it|  it)
            })
            .unwrap();

        a
    }
}

impl<'a, T: Component> Fetch<'a> for &mut T {
    type RawItem = &'a mut T;
    type Item = Vec<(EntityId, Self::RawItem)>;

    fn fetch(scene: &'a Scene) -> Self::Item {
        scene
            .entities
            .iter()
            .filter_map(|entity| {
                entity
                    .get_component_mut::<T>()
                    .ok()
                    .map(|it| (entity.id, it))
            })
            .collect()
    }

    fn fetch_single(scene: &'a Scene) -> Self::RawItem {
        scene
            .entities
            .iter()
            .find_map(|entity| entity.get_component_mut::<T>().ok())
            .unwrap()
    }
}

impl<'a, T: Component> FetchRaw<'a> for &T {
    type RawItem = &'a T;

    fn fetch(entity: &'a Entity) -> Result<Self::RawItem, ComponentError> {
        entity.get_component::<T>()
    }
}

impl<'a, T: Component> FetchRaw<'a> for &mut T {
    type RawItem = &'a mut T;

    fn fetch(entity: &'a Entity) -> Result<Self::RawItem, ComponentError> {
        entity.get_component_mut::<T>()
    }
}

pub type QueryResult<T> = Vec<T>;

impl<'a, A: FetchRaw<'a>> Query<'a> for (A,) {
    type Item = (EntityId, (A::RawItem,));
    fn get_components_in_all_entities(scene: &'a Scene) -> QueryFetched<Self::Item> {
        let mut res = Vec::new();
        for entity in &scene.entities {
            if let Ok(a) = A::fetch(entity) {
                res.push((entity.id, (a,)));
            }
        }

        QueryFetched { result: res }
    }
}

macro_rules! impl_query_for_tuple {
    ($($name: ident),*) => {
        impl<'a, $($name: FetchRaw<'a>),*> Query<'a> for ($($name),*) {
            type Item = (EntityId, ($($name::RawItem),*));
            fn get_components_in_all_entities(scene: &'a Scene) ->  QueryFetched<Self::Item> {
                let mut res = Vec::new();
                for entity in &scene.entities {
                    let matches = $($name::founded(entity))&&*;
                    if matches {
                        res.push((entity.id, ($($name::fetch(entity).unwrap()),*)));
                    }
                }

                QueryFetched {
                    result: res
                }
            }
        }
    };
}

impl_query_for_tuple!(A, B);
impl_query_for_tuple!(A, B, C);
impl_query_for_tuple!(A, B, C, D);
impl_query_for_tuple!(A, B, C, D, E);
impl_query_for_tuple!(A, B, C, D, E, F);
impl_query_for_tuple!(A, B, C, D, E, F, G);
impl_query_for_tuple!(A, B, C, D, E, F, G, H);
impl_query_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_query_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_query_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_query_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
