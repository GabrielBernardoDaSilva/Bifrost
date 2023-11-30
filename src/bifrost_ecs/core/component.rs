use std::{
    any::{Any, TypeId},
    collections::HashSet,
    fmt::Debug,
};

use super::entity;

pub trait Component: 'static + Send + Sync + Debug {}

impl<T: 'static + Send + Sync + Debug> Component for T {}

pub trait AsAny: Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait ComponentBundle {
    fn add_components_to_entity(
        self,
        unique_instances: &HashSet<TypeId>,
        entity: &mut entity::EntityStorage,
    );
}

macro_rules! impl_components_bundle_for_tuples {
    ($(($name: ident, $index: tt)),*) => {
        impl<$($name: Component), *> ComponentBundle for ($($name,)*){
            fn add_components_to_entity(self, unique_instances: &HashSet<TypeId>, entity: &mut entity::EntityStorage) {
                $(
                    // check if component is unique
                    if unique_instances.contains(&TypeId::of::<$name>()) {
                        panic!("Component {:?} is unique istance held by the engine!", stringify!($name));
                    }
                    entity.add_component(self.$index).unwrap();
                )*
            }
        }
    };
}

impl_components_bundle_for_tuples!((A, 0));
impl_components_bundle_for_tuples!((A, 0), (B, 1));
impl_components_bundle_for_tuples!((A, 0), (B, 1), (C, 2));
impl_components_bundle_for_tuples!((A, 0), (B, 1), (C, 2), (D, 3));
impl_components_bundle_for_tuples!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
impl_components_bundle_for_tuples!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
impl_components_bundle_for_tuples!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
impl_components_bundle_for_tuples!(
    (A, 0),
    (B, 1),
    (C, 2),
    (D, 3),
    (E, 4),
    (F, 5),
    (G, 6),
    (H, 7)
);
impl_components_bundle_for_tuples!(
    (A, 0),
    (B, 1),
    (C, 2),
    (D, 3),
    (E, 4),
    (F, 5),
    (G, 6),
    (H, 7),
    (I, 8)
);
impl_components_bundle_for_tuples!(
    (A, 0),
    (B, 1),
    (C, 2),
    (D, 3),
    (E, 4),
    (F, 5),
    (G, 6),
    (H, 7),
    (I, 8),
    (J, 9)
);
impl_components_bundle_for_tuples!(
    (A, 0),
    (B, 1),
    (C, 2),
    (D, 3),
    (E, 4),
    (F, 5),
    (G, 6),
    (H, 7),
    (I, 8),
    (J, 9),
    (K, 10)
);
