use std::{
    any::TypeId,
    collections::{HashMap, VecDeque},
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::core::component::AsAny;

#[derive(Debug)]
pub struct Event<T: EventComponent> {
    data: VecDeque<T>,
}

impl<T: EventComponent> DerefMut for Event<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T: EventComponent> Deref for Event<T> {
    type Target = VecDeque<T>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub trait EventHandle<T: EventComponent> {
    fn send(&mut self, data: T);
    fn read<'a>(&'a mut self) -> Option<T>;
    fn clear(&mut self);
}

impl<T: EventComponent> Event<T> {
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }
}

impl<T: EventComponent> EventHandle<T> for Event<T> {
    fn send(&mut self, data: T) {
        self.data.push_back(data);
    }

    fn read<'a>(&'a mut self) -> Option<T> {
        self.pop_front()
    }

    fn clear(&mut self) {
        self.data.clear();
    }
}

impl<T: EventComponent> AsAny for Event<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub trait EventComponent: 'static + AsAny + Send + Sync {}

#[derive(Debug)]
pub struct EventStorage {
    events: HashMap<TypeId, Box<dyn AsAny + Send + Sync>>,
}

impl EventStorage {
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
        }
    }

    pub fn add_event<T: EventComponent + 'static>(&mut self) {
        self.events
            .insert(TypeId::of::<T>(), Box::new(Event::<T>::new()));
    }

    pub fn send<T: EventComponent + 'static>(&mut self, data: T) {
        let event = self.events.get_mut(&TypeId::of::<T>()).unwrap();
        let event = event.as_any_mut().downcast_mut::<Event<T>>().unwrap();
        event.send(data);
    }

    pub fn read<'a, T: EventComponent + 'static>(
        &'a mut self,
    ) -> Option<T> {
        let event = self.events.get_mut(&TypeId::of::<T>()).unwrap();
        let event = event.as_any_mut().downcast_mut::<Event<T>>().unwrap();
        event.read()
    }

    pub fn clear<T: EventComponent + 'static>(&mut self) {
        let event = self.events.get_mut(&TypeId::of::<T>()).unwrap();
        let event = event.as_any_mut().downcast_mut::<Event<T>>().unwrap();
        event.clear();
    }
}
