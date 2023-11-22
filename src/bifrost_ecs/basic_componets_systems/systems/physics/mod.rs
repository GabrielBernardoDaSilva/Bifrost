use crate::bifrost_ecs::basic_componets_systems::components::graphics::sprite_renderer::SpriteRenderer;

pub fn collision_aabb(sprite: &mut SpriteRenderer, sprite2: &mut SpriteRenderer) -> bool {
    let collission_x = sprite.position.x + sprite.size.x >= sprite2.position.x
        && sprite2.position.x + sprite2.size.x >= sprite.position.x;

    let collission_y = sprite.position.y + sprite.size.y >= sprite2.position.y
        && sprite2.position.y + sprite2.size.y >= sprite.position.y;
    return collission_x && collission_y;
}
