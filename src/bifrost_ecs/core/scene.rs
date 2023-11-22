use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use glfw::Context;
use glow::HasContext;

use crate::bifrost_ecs::{
    inputs::{keys::Keys, mouse::Mouse, Input},
    resources::{model::Model, shader::Shader, texture::Texture, Asset, time::Time, text_renderer::TextRenderer, sound::Sound},
    window::Window,
};

use super::{
    component::{Component, ComponentBundle},
    entity::{Entity, EntityId},
    errors::{ComponentError, ComponentNotFounded},
    lifetime_system_exec::LifetimeSystemExec,
    query::{Fetch, Query, QueryFetched},
};

pub type SystemFunc = (Box<dyn Fn(&mut Scene)>, LifetimeSystemExec);
#[macro_export]
macro_rules! system {
    ($(($func_name: ident, $lifetime_system_exec: expr)),*) => {
        {

            let v: Vec<$crate::core::scene::SystemFunc> = vec![
                $((Box::new($func_name),$lifetime_system_exec),)*
            ];
            v
        }
    };
}


pub struct Scene {
    pub(crate) entities: Vec<Entity>,
    pub(crate) systems: Arc<Mutex<HashMap<LifetimeSystemExec, Vec<Box<dyn Fn(&mut Scene)>>>>>,
    is_running: bool,
    pub window_container: Window,
    unique_instances: HashSet<TypeId>,
}

impl Scene {
    pub fn new() -> Self {
        let mut systems = HashMap::new();
        systems.insert(LifetimeSystemExec::OnBegin, Vec::new());
        systems.insert(LifetimeSystemExec::OnUpdate, Vec::new());
        systems.insert(LifetimeSystemExec::OnFinish, Vec::new());

        let mut scene = Self {
            entities: Vec::new(),
            is_running: false,
            systems: Arc::new(Mutex::new(systems)),
            window_container: Window::new("Prometheus", 800, 600),
            unique_instances: HashSet::new(),
        };

        // resources

        let keys = Keys::new();
        let mouse = Mouse::new();

        scene.spawn((
            Asset::<Shader>::new(),
            Asset::<Texture>::new(),
            Asset::<Model>::new(),
            Asset::<TextRenderer>::new(),
            Asset::<Sound>::new(),
            Input::new(keys),
            Input::new(mouse),
            Time::new(),
        ));

        let mut unique_instances = HashSet::new();
        unique_instances.insert(TypeId::of::<Input<Keys>>());
        unique_instances.insert(TypeId::of::<Input<Mouse>>());
        unique_instances.insert(TypeId::of::<Asset<Shader>>());
        unique_instances.insert(TypeId::of::<Asset<Texture>>());
        unique_instances.insert(TypeId::of::<Asset<Model>>());
        unique_instances.insert(TypeId::of::<Asset<TextRenderer>>());
        unique_instances.insert(TypeId::of::<Time>());
        scene.unique_instances = unique_instances;

        scene
    }

    pub fn query_single<'a, T: Fetch<'a>>(&'a self) -> T::RawItem {
        <T>::fetch_single(self)
    }

    pub fn query<'a, T: Query<'a>>(&'a self) -> QueryFetched<T::Item> {
        <T>::get_components_in_all_entities(self)
    }

    pub fn spawn(&mut self, cb: impl ComponentBundle) -> &mut Self {
        let mut e = Entity::new(self.entities.len() as u32);
        e.add_components(&self.unique_instances, cb);
        self.entities.push(e);
        self
    }

    pub fn spawn_batch(&mut self, cbs: Vec<impl ComponentBundle>) -> &mut Self {
        for cb in cbs {
            self.spawn(cb);
        }
        self
    }

    pub fn remove_entity(&mut self, entity_id: EntityId) {
        self.entities.retain(|e| e.id != entity_id);
    }

    pub fn remove_component_from_entity<T: Component>(&mut self, entity_id: EntityId) {
        if let Some(entity) = self.entities.iter_mut().find(|e| e.id == entity_id) {
            entity.remove_component::<T>();
        }
    }

    pub fn add_components_to_entity(
        &mut self,
        entity_id: EntityId,
        component: impl ComponentBundle,
    ) {
        if let Some(entity) = self.entities.iter_mut().find(|e| e.id == entity_id) {
            entity.add_components(&self.unique_instances, component);
        }
    }

    pub fn add_component_to_entity<T: Component>(
        &mut self,
        entity_id: EntityId,
        component: T,
    ) -> Result<(), ComponentError> {
        if let Some(entity) = self.entities.iter_mut().find(|e| e.id == entity_id) {
            entity.add_component(component)
        } else {
            Err(ComponentError::ComponentNotFoundedError(
                ComponentNotFounded::new::<T>(),
            ))
        }
    }

    pub fn add_system(
        &mut self,
        system: impl Fn(&mut Scene) + 'static,
        exec: LifetimeSystemExec,
    ) -> &mut Self {
        self.systems
            .lock()
            .unwrap()
            .get_mut(&exec)
            .unwrap()
            .push(Box::new(system));
        self
    }

    pub fn add_systems(&mut self, systems: Vec<SystemFunc>) -> &mut Self {
        for (system, exec) in systems {
            self.systems
                .lock()
                .unwrap()
                .get_mut(&exec)
                .unwrap()
                .push(system);
        }
        self
    }

    fn run_system_on_begin(&mut self) {
        let systems = self.systems.clone();
        let lock = systems.lock().unwrap();
        let systems = lock.get(&LifetimeSystemExec::OnBegin).unwrap();
        for system in systems {
            system(self);
        }
    }

    fn run_system_on_update(&mut self) {
        let systems = self.systems.clone();
        let lock = systems.lock().unwrap();
        let system = lock.get(&LifetimeSystemExec::OnUpdate).unwrap();
        for system in system {
            system(self);
        }
    }

    fn run_system_on_finish(&mut self) {
        let systems = self.systems.clone();
        let lock = systems.lock().unwrap();
        let systems = lock.get(&LifetimeSystemExec::OnFinish).unwrap();
        for system in systems {
            system(self);
        }
    }

    pub fn run_systems(&mut self) {
        self.run_system_on_begin();
        self.run_system_on_update();
        self.run_system_on_finish();
    }

    pub fn run_forever(&mut self) {
        self.is_running = true;
        self.run_system_on_begin();
        while self.is_running && !self.window_container.window.should_close() {
            self.window_container.window.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&self.window_container.events) {
                let keys = self.query_single::<&mut Input<Keys>>();
                let mouse = self.query_single::<&mut Input<Mouse>>();
                self.window_container.input_handler(event, keys, mouse);
            }
            unsafe {
                self.window_container.gl.clear(glow::COLOR_BUFFER_BIT);
            };
            self.run_system_on_update();
            self.query_single::<&mut Time>().update();
            
            
            self.window_container.window.swap_buffers();
        }
        self.run_system_on_finish();
    }

    pub fn stop(&mut self) {
        self.is_running = false;
        self.window_container.window.set_should_close(true);
    }
}
