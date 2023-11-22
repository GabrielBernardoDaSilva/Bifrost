use std::{collections::HashMap, sync::Arc};

use crate::bifrost_ecs::{resources::Asset, core::scene};

use super::{shader::Shader, AssetLoader};
use glow::HasContext;
use nalgebra_glm as glm;

#[derive(Debug)]
pub struct Character {
    texture_id: glow::NativeTexture,
    size: glm::Vec2,
    bearing: glm::Vec2,
    advance: u32,
}

#[derive(Debug)]
pub struct TextRenderer {
    pub characters: HashMap<char, Character>,
    pub shader: Arc<Shader>,
    vao: glow::NativeVertexArray,
    vbo: glow::NativeBuffer,
}

impl AssetLoader for TextRenderer {
    type Asset = Self;

    fn load(
        scene: &crate::bifrost_ecs::core::scene::Scene,
        path: &str,
    ) -> Result<std::sync::Arc<Self::Asset>, super::asset_loader_errors::AssetLoaderError> {
        // create text renderer
        let gl = scene.window_container.gl.clone();
        let shaders = scene.query_single::<&mut Asset<Shader>>();
        let shader = shaders.get("text_2d").unwrap().clone();

        shader.use_program(gl.clone());
        shader.set_mat4(
            "u_projection",
            &glm::ortho(0.0, 800 as f32, 0.0, 600 as f32, -1.0, 1.0),
            gl.clone(),
        );
        shader.set_i32("u_text", 0, gl.clone());
        let vao = unsafe {
            let vao = gl.create_vertex_array().unwrap();
            vao
        };
        let vbo = unsafe {
            gl.bind_vertex_array(Some(vao));
            let vbo = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_size(
                glow::ARRAY_BUFFER,
                (6 * 4 * std::mem::size_of::<f32>()) as i32,
                glow::DYNAMIC_DRAW,
            );
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(
                0,
                4,
                glow::FLOAT,
                false,
                4 * std::mem::size_of::<f32>() as i32,
                0,
            );
            vbo
        };
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_vertex_array(None);
        }
        //

        let mut characters = HashMap::new();

        let ft = freetype::Library::init().unwrap();
        let face = ft.new_face(path, 0).unwrap();
        let font_size = 24;
        face.set_pixel_sizes(0, font_size).unwrap();
        unsafe {
            gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 1);

            for c in 0..128 {
                face.load_char(c as usize, freetype::face::LoadFlag::RENDER)
                    .unwrap();
     


          
                let texture = gl.create_texture().unwrap();
                gl.bind_texture(glow::TEXTURE_2D, Some(texture));
                gl.tex_image_2d(
                    glow::TEXTURE_2D,
                    0,
                    glow::RED as i32,
                    face.glyph().bitmap().width(),
                    face.glyph().bitmap().rows(),
                    0,
                    glow::RED,
                    glow::UNSIGNED_BYTE,
                    Some(face.glyph().bitmap().buffer()),
                );
                gl.tex_parameter_i32(
                    glow::TEXTURE_2D,
                    glow::TEXTURE_WRAP_S,
                    glow::CLAMP_TO_EDGE as i32,
                );
                gl.tex_parameter_i32(
                    glow::TEXTURE_2D,
                    glow::TEXTURE_WRAP_T,
                    glow::CLAMP_TO_EDGE as i32,
                );
                gl.tex_parameter_i32(
                    glow::TEXTURE_2D,
                    glow::TEXTURE_MIN_FILTER,
                    glow::LINEAR as i32,
                );
                gl.tex_parameter_i32(
                    glow::TEXTURE_2D,
                    glow::TEXTURE_MAG_FILTER,
                    glow::LINEAR as i32,
                );


                let character = Character {
                    texture_id: texture,
                    size: glm::vec2(
                        face.glyph().bitmap().width() as f32,
                        face.glyph().bitmap().rows() as f32,
                    ),
                    bearing: glm::vec2(
                        face.glyph().bitmap_left() as f32,
                        face.glyph().bitmap_top() as f32,
                    ),
                    advance: face.glyph().advance().x as u32,
                };

                characters.insert((c as u8) as char, character);
            }

            gl.bind_texture(glow::TEXTURE_2D, None);
            
        };

        Ok(Arc::new(Self {
            characters,
            shader,
            vao,
            vbo,
        }))
    }
}

impl TextRenderer {
    pub fn render_text(&self, scene: &mut scene::Scene, text: &str, x: f32, y: f32, scale: f32, color: glm::Vec4) {
        let gl = scene.window_container.gl.clone();
        self.shader.use_program(gl.clone());
        self.shader.set_vec4("u_text_color", &color, gl.clone());
        let mut x = x;
        unsafe{

            gl.active_texture(glow::TEXTURE0);
            gl.bind_vertex_array(Some(self.vao));
            for c in text.as_bytes(){
                let char = self.characters.get(&(*c as char)).unwrap();
                let xpos = x + char.bearing.x * scale;
                let ypos = y - (char.size.y - char.bearing.y) * scale;

                let w = char.size.x * scale;
                let h = char.size.y * scale;

                let vertices: [f32; 24] = [
                    xpos, ypos + h, 0.0, 0.0,
                    xpos, ypos, 0.0, 1.0,
                    xpos + w, ypos, 1.0, 1.0,

                    xpos, ypos + h, 0.0, 0.0,
                    xpos + w, ypos, 1.0, 1.0,
                    xpos + w, ypos + h, 1.0, 0.0,
                ];

                gl.bind_texture(glow::TEXTURE_2D, Some(char.texture_id));
                gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
                gl.buffer_sub_data_u8_slice(glow::ARRAY_BUFFER, 0, bytemuck::cast_slice(&vertices));
                gl.bind_buffer(glow::ARRAY_BUFFER, None);

                gl.draw_arrays(glow::TRIANGLES, 0, 6);

                x += (char.advance >> 6) as f32 * scale;

            }
            gl.bind_vertex_array(None);
            gl.bind_texture(glow::TEXTURE_2D, None);
        }
    }
}
