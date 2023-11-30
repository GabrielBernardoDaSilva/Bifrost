use bifrost_ecs::{
    basic_componets_systems::{
        components::graphics::{material::Material, quad::Quad, sprite_renderer::SpriteRenderer},
        systems::{graphics::render_sprite, physics::collision_aabb},
    },
    core::{
        component::AsAny, entity::Entity, lifetime_system_exec::LifetimeSystemExec, scene::Scene,
    },
    inputs::{keys::Keys, Input},
    resources::{
        event::EventComponent, shader::Shader, sound::Sound, text_renderer::TextRenderer,
        texture::Texture, time::Time, Asset,
    },
    system, system_mut,
};
use glfw::Key;
use nalgebra_glm as glm;
use rand::Rng;

#[derive(Debug)]
struct Name(String);

#[derive(Debug)]
struct Velocity(f32, f32);

#[derive(Debug)]
struct PaddleVelocity(f32);

#[derive(Debug)]
struct GameScore {
    winner: Option<String>,
    amount_to_wait: f32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum GameState {
    Menu,
    Playing,
}

#[derive(Debug)]
struct GameStateSystem(GameState);

#[derive(Debug)]
struct CollisionEvent(glm::Vec4);

impl EventComponent for CollisionEvent {}

impl AsAny for CollisionEvent {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

fn main() {
    let mut scene = Scene::new();
    scene.spawn((GameStateSystem(GameState::Menu),));

    scene.spawn((GameScore {
        winner: None,
        amount_to_wait: 5.0,
    },));

    scene
        .add_event::<CollisionEvent>()
        .add_systems(system!(
            (load_assets, LifetimeSystemExec::OnBegin),
            (setup_shader_projection, LifetimeSystemExec::OnBegin),
            (create_sprites, LifetimeSystemExec::OnBegin)
        ))
        .add_systems(system!(
            (occlusion_sprites, LifetimeSystemExec::OnUpdate),
            (move_paddle_player, LifetimeSystemExec::OnUpdate),
            (render_sprite, LifetimeSystemExec::OnUpdate),
            (ball_movement, LifetimeSystemExec::OnUpdate),
            (ball_collision, LifetimeSystemExec::OnUpdate),
            (ball_collision_event, LifetimeSystemExec::OnUpdate),
            (check_winner, LifetimeSystemExec::OnUpdate),
            (check_if_is_wait_screen, LifetimeSystemExec::OnUpdate)
        ))
        .add_mut_systems(system_mut!(
            (draw_menu, LifetimeSystemExec::OnUpdate),
            (render_text, LifetimeSystemExec::OnUpdate),
            (control_game_state, LifetimeSystemExec::OnUpdate),
            (set_fps_in_window_title, LifetimeSystemExec::OnUpdate),
            (water_mark, LifetimeSystemExec::OnUpdate)
        ));

    scene.run_forever();
}

fn water_mark(scene: &mut Scene) {
    let text_renderer = scene.query_single::<&mut Asset<TextRenderer>>();
    let text_renderer = text_renderer.get("Antonio-Regular").unwrap();
    text_renderer.render_text(
        scene,
        "Bifrost ECS",
        10.0,
        575.0,
        0.9,
        glm::vec4(0.1, 0.3, 1.0, 1.0),
    );

    text_renderer.render_text(
        scene,
        "Press esc to exit",
        700.0,
        580.0,
        0.5,
        glm::vec4(1.0, 1.0, 1.0, 1.0),
    );
}

fn set_fps_in_window_title(scene: &mut Scene) {
    let time = scene.query_single::<&Time>().clone();
    scene.window_container.window.set_title(&format!(
        "Pong - FPS: {:0}",
        (1.0 / time.delta_time.as_secs_f32()) as i32
    ));
}

fn control_game_state(scene: &mut Scene) {
    let keys = scene.query_single::<&Input<Keys>>();
    if let Some(action) = keys.get_key(Key::Space) {
        if action == &glfw::Action::Press {
            scene.query_single::<&mut GameStateSystem>().0 = GameState::Playing;
        }
    }

    if let Some(action) = keys.get_key(Key::Escape) {
        if action == &glfw::Action::Press {
            scene.stop();
        }
    }
}

fn draw_menu(scene: &mut Scene) {
    if scene.query_single::<&GameStateSystem>().0 != GameState::Menu {
        return;
    }

    let text_renderer = scene.query_single::<&mut Asset<TextRenderer>>();
    let text_renderer = text_renderer.get("Antonio-Regular").unwrap();
    let time = scene.query_single::<&Time>().clone();
    text_renderer.render_text(
        scene,
        "Pong",
        380.0,
        350.0,
        1.1,
        glm::vec4(1.0, 1.0, 1.0, 1.0),
    );

    text_renderer.render_text(
        scene,
        "Press Space to start",
        330.0,
        250.0,
        0.8,
        glm::vec4(1.0, time.time.sin(), time.time.cos(), 1.0),
    );
}

fn occlusion_sprites(scene: &Scene) {
    if scene.query_single::<&GameStateSystem>().0 == GameState::Menu {
        let mut query = scene.query::<(&mut SpriteRenderer,)>();
        for (sprite,) in query.iter_mut() {
            sprite.material.color.w = 0.0;
        }
    } else {
        let mut query = scene.query::<(&mut SpriteRenderer,)>();
        for (sprite,) in query.iter_mut() {
            sprite.material.color.w = 1.0;
        }
    }
}

fn check_if_is_wait_screen(scene: &Scene) {
    let game_score = scene.query_single::<&mut GameScore>();
    let time = scene.query_single::<&Time>();
    let mut query = scene.query::<(&mut SpriteRenderer, &Name)>();
    if game_score.winner.is_some() {
        game_score.amount_to_wait -= time.delta_time.as_secs_f32();
        println!("{}", game_score.amount_to_wait);
        if game_score.amount_to_wait <= 0.0 {
            game_score.winner = None;
            game_score.amount_to_wait = 5.0;
            for (sprite, name) in query.iter_mut() {
                if name.0 == "Ball" {
                    sprite.material.color = glm::vec4(1.0, 1.0, 1.0, 1.0);
                }
            }
        }
    }
}

fn load_assets(scene: &Scene) {
    let asset_shader = scene.query_single::<&mut Asset<Shader>>();
    asset_shader
        .load(
            &scene,
            "assets/shaders/default.vert;assets/shaders/default.frag",
        )
        .expect("Error loading shader");

    asset_shader
        .load(
            &scene,
            "assets/shaders/text_2d.vert;assets/shaders/text_2d.frag",
        )
        .expect("Error loading shader");

    let asset_texture = scene.query_single::<&mut Asset<Texture>>();
    asset_texture
        .load(&scene, "assets/textures/ball.png")
        .expect("Error loading texture");
    asset_texture
        .load(scene, "assets/textures/quad.png")
        .unwrap();

    let asset_font = scene.query_single::<&mut Asset<TextRenderer>>();
    asset_font
        .load(&scene, "assets/fonts/Antonio-Regular.ttf")
        .expect("Error loading font");

    let asset_sound = scene.query_single::<&mut Asset<Sound>>();
    asset_sound
        .load(&scene, "assets/sounds/pong.wav")
        .expect("Error loading sound");
}
fn create_sprites(scene: &Scene) {
    let textures_asset = scene.query_single::<&Asset<Texture>>();
    let shader_asset = scene.query_single::<&Asset<Shader>>();

    let paddle_tex = textures_asset.get("quad").unwrap();
    let ball_tex = textures_asset.get("ball").unwrap();

    let sprite_shader = shader_asset.get("default").unwrap();

    // ball
    let ball_material = Material {
        color: glm::vec4(1.0, 1.0, 1.0, 1.0),
        shader: sprite_shader.clone(),
        texture: ball_tex,
    };
    let ball_quad = Quad::new(scene.window_container.gl.clone());

    let ball_sprite = SpriteRenderer::new(
        ball_quad,
        ball_material,
        glm::vec2(400.0, 300.0),
        glm::vec2(10.0, 10.0),
        0.0,
    );
    let ball_velocity = Velocity(0.0, 200.0);

    // paddle 1

    let paddle_quad_1 = Quad::new(scene.window_container.gl.clone());
    let paddle_material_1 = Material {
        color: glm::vec4(1.0, 0.0, 0.0, 1.0),
        shader: sprite_shader.clone(),
        texture: paddle_tex.clone(),
    };

    let paddle1_sprite = SpriteRenderer::new(
        paddle_quad_1,
        paddle_material_1,
        glm::vec2(350.0, 50.0),
        glm::vec2(100.0, 10.0),
        0.0,
    );

    // paddle 2

    let paddle_quad_2 = Quad::new(scene.window_container.gl.clone());
    let paddle_material_2 = Material {
        color: glm::vec4(0.0, 1.0, 0.0, 1.0),
        shader: sprite_shader,
        texture: paddle_tex,
    };

    let paddle2_sprite = SpriteRenderer::new(
        paddle_quad_2,
        paddle_material_2,
        glm::vec2(350.0, 550.0),
        glm::vec2(100.0, 10.0),
        0.0,
    );

    scene.spawn((ball_sprite, Name("Ball".to_owned()), ball_velocity));

    scene.spawn((
        paddle1_sprite,
        Name("Block1".to_owned()),
        PaddleVelocity(100.0),
    ));

    scene.spawn((
        paddle2_sprite,
        Name("Block2".to_owned()),
        PaddleVelocity(100.0),
    ));
}

fn ball_movement(scene: &Scene) {
    let game_state = scene.query_single::<&mut GameStateSystem>();
    if game_state.0 == GameState::Menu {
        return;
    }

    let mut query = scene.query::<(&mut SpriteRenderer, &Name, &mut Velocity)>();
    let time = scene.query_single::<&Time>();
    let sound = scene.query_single::<&Asset<Sound>>().get("pong").unwrap();
    for (sprite, name, velocity) in query.iter_mut() {
        if name.0 == "Ball" {
            sprite.position.x += velocity.0 * time.delta_time.as_secs_f32();
            sprite.position.y += velocity.1 * time.delta_time.as_secs_f32();
            if sprite.position.x > 800.0 || sprite.position.x < 0.0 {
                sound.play();
                velocity.0 *= -1.0;
            }
        }
    }
}

fn ball_collision_event(scene: &Scene) {
    let game_state = scene.query_single::<&mut GameStateSystem>();
    if game_state.0 == GameState::Menu {
        return;
    }

    let last_item = scene.read_event::<CollisionEvent>();

    if let Some(event) = last_item {
        let mut ball_query = scene.query::<(&mut SpriteRenderer, &Name, &mut Velocity)>();
        let sound = scene.query_single::<&Asset<Sound>>().get("pong").unwrap();
        for (sprite, name, velocity) in ball_query.iter_mut() {
            if name.0 == "Ball" {
                velocity.1 *= -1.0;
                velocity.0 = (rand::random::<f32>() * 100.0) - 50.0;
                sprite.material.color = event.0;
                sound.play();
            }
        }
    }
}

fn ball_collision(scene: &Scene) {
    let game_state = scene.query_single::<&mut GameStateSystem>();
    if game_state.0 == GameState::Menu {
        return;
    }

    let mut ball_query = scene.query::<(&mut SpriteRenderer, &Name, &mut Velocity, &Entity)>();
    let mut query = scene.query::<(&mut SpriteRenderer, &Name, &Entity)>();

    for (sprite, name, _, id) in ball_query.iter_mut() {
        if name.0 == "Ball" {
            for (sprite2, name2, id2) in query.iter_mut() {
                if id == id2 {
                    continue;
                }
                if name2.0 == "Block1" || name2.0 == "Block2" {
                    if collision_aabb(sprite, sprite2) {
                        scene.send_event(CollisionEvent(sprite2.material.color));
                    }
                }
            }
        }
    }
}

fn move_paddle_player(scene: &Scene) {
    let game_state = scene.query_single::<&mut GameStateSystem>();
    if game_state.0 == GameState::Menu {
        return;
    }
    let mut paddle_query = scene.query::<(&mut SpriteRenderer, &Name, &PaddleVelocity)>();
    let time = scene.query_single::<&Time>();
    for (sprite, name, paddle_velocity) in paddle_query.iter_mut() {
        if name.0 == "Block1" {
            if let Some(action) = scene.query_single::<&Input<Keys>>().get_key(Key::Left) {
                if (action == &glfw::Action::Press || action == &glfw::Action::Repeat)
                    && sprite.position.x > 0.0
                {
                    sprite.position.x -= paddle_velocity.0 * time.delta_time.as_secs_f32();
                }
            }
            if let Some(action) = scene.query_single::<&Input<Keys>>().get_key(Key::Right) {
                if (action == &glfw::Action::Press || action == &glfw::Action::Repeat)
                    && sprite.position.x < 800.0 - sprite.size.x
                {
                    sprite.position.x += paddle_velocity.0 * time.delta_time.as_secs_f32();
                }
            }
        }
        if name.0 == "Block2" {
            if let Some(action) = scene.query_single::<&Input<Keys>>().get_key(Key::A) {
                if (action == &glfw::Action::Press || action == &glfw::Action::Repeat)
                    && sprite.position.x > 0.0
                {
                    sprite.position.x -= paddle_velocity.0 * time.delta_time.as_secs_f32();
                }
            }
            if let Some(action) = scene.query_single::<&Input<Keys>>().get_key(Key::D) {
                if (action == &glfw::Action::Press || action == &glfw::Action::Repeat)
                    && sprite.position.x < 800.0 - sprite.size.x
                {
                    sprite.position.x += paddle_velocity.0 * time.delta_time.as_secs_f32();
                }
            }
        }
    }
}

fn setup_shader_projection(scene: &Scene) {
    let shader = scene.query_single::<&mut Asset<Shader>>();
    let shader = shader.get("default").unwrap();
    let ortho = glm::ortho(0.0, 800.0, 0.0, 600.0, -1.0, 1.0);
    shader.use_program(scene.window_container.gl.clone());
    shader.set_mat4("u_projection", &ortho, scene.window_container.gl.clone());
}

fn check_winner(scene: &Scene) {
    let game_state = scene.query_single::<&mut GameStateSystem>();
    if game_state.0 == GameState::Menu {
        return;
    }
    let mut query = scene.query::<(&SpriteRenderer, &Name)>();
    let game_score = scene.query_single::<&mut GameScore>();
    for (sprite, name) in query.iter_mut() {
        if name.0 == "Ball" {
            if sprite.position.y > 600.0 {
                game_score.winner = Some("Block1".to_owned());
            } else if sprite.position.y < 0.0 {
                game_score.winner = Some("Block2".to_owned());
            }
        }
    }
}

fn render_text(scene: &mut Scene) {
    let text_renderer: &mut Asset<TextRenderer> = scene.query_single::<&mut Asset<TextRenderer>>();
    let text_renderer = text_renderer.get("Antonio-Regular").unwrap();
    let game_score = scene.query_single::<&GameScore>();
    if game_score.winner.is_some() {
        let result = game_score.winner.as_ref().unwrap();
        text_renderer.render_text(
            scene,
            &format!("Winner: {}", result),
            300.0,
            300.0,
            1.0,
            if result == "Block1" {
                glm::vec4(1.0, 0.0, 0.0, 1.0)
            } else {
                glm::vec4(0.0, 1.0, 0.0, 1.0)
            },
        );

        let mut query = scene.query::<(&mut SpriteRenderer, &Name, &mut Velocity)>();
        for (sprite, name, velocity) in query.iter_mut() {
            if name.0 == "Ball" {
                sprite.position = glm::vec2(400.0, 300.0);
                sprite.material.color = glm::vec4(1.0, 1.0, 1.0, 0.0);
                // rand true or false
                let mut rng = rand::thread_rng();
                let x = rng.gen_range(0..=1);

                velocity.0 = 0.0;
                velocity.1 = if x == 0 { 100.0 } else { -100.0 };
            }
        }
    }
}


