use glow::HasContext;

use crate::bifrost_ecs::{
    basic_componets_systems::components::graphics::sprite_renderer::SpriteRenderer,
    core::scene::Scene,
};

use nalgebra_glm as glm;

pub fn render_sprite(scene: &Scene) {
    let gl = &scene.window_container.gl;
    let query_sprite = scene.query::<(&mut SpriteRenderer,)>();
    for (_, (sprite,)) in query_sprite.iter() {
        sprite.material.use_material(gl.clone());

        let mut model = glm::Mat4::identity();
        model = glm::translate(
            &model,
            &glm::vec3(sprite.position.x, sprite.position.y, 0.0),
        );

        model = glm::translate(
            &model,
            &glm::vec3(0.5 * sprite.size.x, 0.5 * sprite.size.y, 0.0),
        );
        model = glm::rotate(&model, sprite.rotation, &glm::vec3(0.0, 0.0, 1.0));
        model = glm::translate(
            &model,
            &glm::vec3(-0.5 * sprite.size.x, -0.5 * sprite.size.y, 0.0),
        );

        model = glm::scale(&model, &glm::vec3(sprite.size.x, sprite.size.y, 1.0));

        sprite
            .material
            .shader
            .set_mat4("u_model", &model, gl.clone());
        sprite
            .material
            .shader
            .set_vec4("u_color", &sprite.material.color, gl.clone());
        sprite.quad.render(gl.clone());

        unsafe {
            gl.bind_vertex_array(None);
        }
    }
}
