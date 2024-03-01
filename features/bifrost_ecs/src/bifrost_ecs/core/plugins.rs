use super::scene;

pub trait Plugin {
    fn build_plugin(&self, scene: &mut scene::Scene);
}
