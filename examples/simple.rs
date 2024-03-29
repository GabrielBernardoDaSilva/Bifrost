use bifrost_derives::EventComponent;
use bifrost_ecs::core::{
    countdown::{self, Timer, TimerCallback},
    scene,
};


struct Name(String);

struct Position(f32, f32);

pub struct Explosion {}

impl TimerCallback for Explosion {
    fn on_timer_finished(&mut self, scene: &mut scene::Scene) {
        println!("Explosion!");
        scene.send_event(CollisionEvent { is_colliding: true });
    }
}

#[derive(EventComponent)]
pub struct CollisionEvent {
    pub is_colliding: bool,
}

fn query(scene: &mut scene::Scene) {
    let query = scene.query::<(&Name, &Position)>();
    for (name, position) in query.iter() {
        println!("Name: {:?}, Position: {:?}", name.0, position.0);
    }

    scene.add_event::<CollisionEvent>();
    scene.send_event(CollisionEvent { is_colliding: true });
}

fn check_for_events(scene: &mut scene::Scene) {
    let event = scene.read_event::<CollisionEvent>();
    if let Some(event) = event {
        println!("Event: {:?}", event.is_colliding);
    }
}

fn main() {
    let mut scene = scene::Scene::new();
    scene.spawn((Name("Paddle".to_string()), Position(0.0, 0.0)));
    scene
        .add_mut_system(
            query,
            bifrost_ecs::core::lifetime_system_exec::LifetimeSystemExec::OnBegin,
        )
        .add_mut_system(
            check_for_events,
            bifrost_ecs::core::lifetime_system_exec::LifetimeSystemExec::OnUpdate,
        );
    scene.countdowns.try_write().unwrap().add_timer(Timer::new(
        "Explosion",
        2.0,
        countdown::TimerScheduler::Once,
        Some(Box::new(Explosion {})),
    ));
    scene.run_forever();
}
