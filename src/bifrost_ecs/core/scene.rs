use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex, RwLock},
};

use glfw::Context;
use glow::HasContext;

use crate::{
    bifrost_ecs::{
        inputs::{keys::Keys, mouse::Mouse, Input},
        resources::{
            model::Model, shader::Shader, sound::Sound, text_renderer::TextRenderer,
            texture::Texture, time::Time, Asset,
        },
        window::Window,
    },
    resources::event::{EventComponent, EventStorage},
};

use super::{
    archetype::Archetype,
    component::{Component, ComponentBundle},
    entity::{Entity, EntityId, EntityStorage, FetchItem},
    errors::ArchetypeError,
    lifetime_system_exec::LifetimeSystemExec,
    query::{Fetch, Query, QueryFetched},
};

pub type SystemFunc = (Box<dyn Fn(&Scene)>, LifetimeSystemExec);
pub type MutSystemFunc = (Box<dyn FnMut(&mut Scene)>, LifetimeSystemExec);
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
#[macro_export]
macro_rules! system_mut {
    ($(($func_name: ident, $lifetime_system_exec: expr)),*) => {
        {

            let v: Vec<$crate::core::scene::MutSystemFunc> = vec![
                $((Box::new($func_name),$lifetime_system_exec),)*
            ];
            v
        }
    };
}

pub struct Scene {
    pub(crate) archetype: Arc<RwLock<Archetype>>,
    pub(crate) systems: Arc<Mutex<HashMap<LifetimeSystemExec, Vec<Box<dyn Fn(&Scene)>>>>>,
    pub(crate) systems_mut:
        Arc<Mutex<HashMap<LifetimeSystemExec, Vec<Box<dyn FnMut(&mut Scene)>>>>>,
    pub events: Arc<RwLock<EventStorage>>,
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

        let mut systems_mut = HashMap::new();
        systems_mut.insert(LifetimeSystemExec::OnBegin, Vec::new());
        systems_mut.insert(LifetimeSystemExec::OnUpdate, Vec::new());
        systems_mut.insert(LifetimeSystemExec::OnFinish, Vec::new());

        let mut scene = Self {
            archetype: Arc::new(RwLock::new(Archetype::new())),
            is_running: false,
            systems: Arc::new(Mutex::new(systems)),
            systems_mut: Arc::new(Mutex::new(systems_mut)),
            window_container: Window::new("Prometheus", 800, 600),
            unique_instances: HashSet::new(),
            events: Arc::new(RwLock::new(EventStorage::new())),
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
        let archetype_arc = self.archetype.clone();
        let mut read = archetype_arc.try_read().unwrap();
        let archetype = read.inner();
        let archetype = unsafe { std::mem::transmute::<&Archetype, &'a Archetype>(archetype) };
        archetype.query_single::<T>()
    }

    pub fn query<'a, T: Query<'a>>(&'a self) -> QueryFetched<T::Item> {
        let archetype_arc = self.archetype.clone();
        let mut read = archetype_arc.try_read().unwrap();
        let archetype = read.inner();
        let archetype = unsafe { std::mem::transmute::<&Archetype, &'a Archetype>(archetype) };

        // Then you can call the `run` method on `tst` like this:
        archetype.query::<T>()
    }

    pub fn spawn(&self, cb: impl ComponentBundle) {
        let archetype_arc = self.archetype.clone();
        let mut archetype = archetype_arc.try_write().unwrap();
        let mut e = EntityStorage::new(archetype.len() as u32);
        e.add_components(&self.unique_instances, cb);
        e.add_component(Entity(e.id)).unwrap();
        archetype.spawn(e);
    }

    pub fn spawn_batch(&mut self, cbs: Vec<impl ComponentBundle>) -> &mut Self {
        for cb in cbs {
            self.spawn(cb);
        }
        self
    }

    pub fn remove_entity(&mut self, entity_id: EntityId) {
        self.archetype
            .clone()
            .try_write()
            .unwrap()
            .remove_entity(entity_id);
    }

    pub fn remove_component_from_entity<T: Component>(&mut self, entity_id: EntityId) {
        self.archetype
            .clone()
            .try_write()
            .unwrap()
            .remove_component_from_entity::<T>(entity_id);
    }

    pub fn add_component_to_entity<T: Component>(
        &mut self,
        entity_id: EntityId,
        component: T,
    ) -> Result<(), ArchetypeError> {
        let archetype_arc = self.archetype.clone();
        let mut archetype = archetype_arc.try_write().unwrap();
        archetype.add_component_to_entity(entity_id, component)?;
        Ok(())
    }

    pub fn add_system(
        &mut self,
        system: impl Fn(&Scene) + 'static,
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

    pub fn add_mut_system(
        &mut self,
        system: impl FnMut(&mut Scene) + 'static,
        exec: LifetimeSystemExec,
    ) -> &mut Self {
        self.systems_mut
            .lock()
            .unwrap()
            .get_mut(&exec)
            .unwrap()
            .push(Box::new(system));
        self
    }

    pub fn add_mut_systems(&mut self, systems: Vec<MutSystemFunc>) -> &mut Self {
        for (system, exec) in systems {
            self.systems_mut
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

        let systems_mut = self.systems_mut.clone();
        let mut lock = systems_mut.lock().unwrap();
        let system = lock.get_mut(&LifetimeSystemExec::OnBegin).unwrap();
        for system in system {
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

        let systems_mut = self.systems_mut.clone();
        let mut lock = systems_mut.lock().unwrap();
        let system = lock.get_mut(&LifetimeSystemExec::OnUpdate).unwrap();
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

        let systems_mut = self.systems_mut.clone();
        let mut lock = systems_mut.lock().unwrap();
        let system = lock.get_mut(&LifetimeSystemExec::OnFinish).unwrap();
        for system in system {
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

    pub fn add_event<T: EventComponent>(&mut self) -> &mut Self {
        let events = self.events.clone();
        events.try_write().unwrap().add_event::<T>();
        self
    }

    pub fn send_event<T: EventComponent>(&self, data: T) {
        let events = self.events.clone();
        events.try_write().unwrap().send::<T>(data);
    }

    pub fn read_event<T: EventComponent>(&self) -> Option<T> {
        self.events.clone().try_write().unwrap().read::<T>()
    }

    pub fn clear_event<T: EventComponent>(&self) {
        let events = self.events.clone();
        events.try_write().unwrap().clear::<T>();
    }
}
