use std::sync::Arc;

use glow::HasContext;

#[derive(Debug)]
pub struct Quad {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub vao: glow::NativeVertexArray,
    pub vbo: glow::NativeBuffer,
    pub ebo: glow::NativeBuffer,
}

impl Quad {
    pub fn new(gl: Arc<glow::Context>) -> Self {
        let vao = unsafe { gl.create_vertex_array().unwrap() };
        let vbo = unsafe { gl.create_buffer().unwrap() };
        let ebo = unsafe { gl.create_buffer().unwrap() };

        let vertices = vec![
            1.0f32, 1.0f32, 0.0f32, 1.0f32, 1.0f32, // top right
            1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, // bottom right
            0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, // bottom lef32t
            0.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, // top left
        ];

        let indices = vec![0u32, 1, 3, 1, 2, 3];

        unsafe {
            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));

            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&vertices),
                glow::STATIC_DRAW,
            );
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(
                0,
                3,
                glow::FLOAT,
                false,
                (std::mem::size_of::<f32>() * 5) as i32,
                0,
            );
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(
                1,
                2,
                glow::FLOAT,
                false,
                (std::mem::size_of::<f32>() * 5) as i32,
                (3 * std::mem::size_of::<f32>()) as i32,
            );

            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&indices),
                glow::STATIC_DRAW,
            );
            gl.bind_vertex_array(None);
        };

        Self {
            vertices,
            indices,
            vao,
            vbo,
            ebo,
            // gl
        }
    }

    pub fn render(&self, gl: Arc<glow::Context>) {
        unsafe {
            gl.bind_vertex_array(Some(self.vao));
            gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
            gl.bind_vertex_array(None);
        }
    }
}
