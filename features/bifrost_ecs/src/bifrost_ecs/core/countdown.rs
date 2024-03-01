use std::fmt::Debug;

use super::scene;

#[derive(Debug)]
pub enum TimerScheduler {
    Once,
    Repeat,
}

pub trait TimerCallback {
    fn on_timer_finished(&mut self, scene: &mut scene::Scene);
}

pub struct Timer {
    pub name: String,
    pub time: f32,
    pub duration: f32,
    pub scheduler: TimerScheduler,
    pub callback: Option<Box<dyn TimerCallback + Send + Sync>>,
    pub mark_destroy: bool,
}

impl Debug for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Timer")
            .field("time", &self.time)
            .field("duration", &self.duration)
            .field("scheduler", &self.scheduler)
            .finish()
    }
}

impl Timer {
    pub fn new(
        name: &str,
        duration: f32,
        scheduler: TimerScheduler,
        callback: Option<Box<dyn TimerCallback + Send + Sync>>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            time: 0.0,
            duration,
            scheduler,
            callback,
            mark_destroy: false,
        }
    }

    pub fn update(&mut self, delta_time: f32, scene: &mut scene::Scene) {
        self.time += delta_time;
        if self.time >= self.duration {
            if let Some(callback) = &mut self.callback {
                callback.on_timer_finished(scene);
            }
            match self.scheduler {
                TimerScheduler::Once => self.mark_destroy = true,
                TimerScheduler::Repeat => self.time = 0.0,
            }
        }
    }

    pub fn stop(&mut self) {
        self.mark_destroy = true;
    }
}

#[derive(Debug)]
pub struct Timers {
    pub timers: Vec<Timer>,
}

impl Timers {
    pub fn new() -> Self {
        Self { timers: Vec::new() }
    }

    pub fn update(&mut self, delta_time: f32, scene: &mut scene::Scene) {
        for timer in &mut self.timers {
            timer.update(delta_time, scene);
        }
        // remove
        self.timers.retain(|timer| !timer.mark_destroy);
    }

    pub fn add_timer(&mut self, timer: Timer) {
        self.timers.push(timer);
    }

    pub fn stop_timer(&mut self, name: &str) {
        self.timers
            .iter_mut()
            .find(|timer| timer.name == name)
            .map(|timer| timer.stop());
    }
}
