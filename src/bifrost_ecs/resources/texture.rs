use std::sync::Arc;

use glow::HasContext;

use crate::bifrost_ecs::core::scene::Scene;

use super::AssetLoader;

#[derive(Debug)]
pub struct Texture {
    pub id: glow::NativeTexture,
}

impl Texture {
    pub fn bind(&self, texture_location: u32, gl: Arc<glow::Context>) {
        unsafe {
            gl.active_texture(glow::TEXTURE0 + texture_location);
            gl.bind_texture(glow::TEXTURE_2D, Some(self.id));
        }
    }
}

impl AssetLoader for Texture {
    type Asset = Self;

    fn load(
        scene: &Scene,
        path: &str,
    ) -> Result<std::sync::Arc<Self::Asset>, super::asset_loader_errors::AssetLoaderError> {
        let gl = scene.window_container.gl.clone();
        let id = unsafe {
            let id = gl.create_texture().expect("Create texture");
            gl.bind_texture(glow::TEXTURE_2D, Some(id));
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);

            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR_MIPMAP_LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );

            let image = image::open(path)
                .expect("Open image")
                .flipv()
                .into_rgba8();
            let (width, height) = image.dimensions();
            let data = image.into_raw();

            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                width as i32,
                height as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                Some(bytemuck::cast_slice(&data)),
            );

            gl.generate_mipmap(glow::TEXTURE_2D);
            gl.bind_texture(glow::TEXTURE_2D, None);
            id
        };

        let texture = Texture {
            id: id,
        };
        Ok(std::sync::Arc::new(texture))
    }
}
