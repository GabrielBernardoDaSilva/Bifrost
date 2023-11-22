use std::sync::Arc;

use glfw::Context;
use glow::HasContext;

use super::inputs::{keys::Keys, mouse::Mouse};

#[allow(dead_code)]
pub struct Window {
    title: String,
    width: u32,
    height: u32,
    resizable: bool,
    vsync: bool,
    fullscreen: bool,
    cursor: bool,
    cursor_grabbed: bool,
    pub glfw: glfw::Glfw,
    pub window: glfw::PWindow,
    pub events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    pub gl: Arc<glow::Context>,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let mut glfw = glfw::init(Self::error_callback).expect("Failed to initialize GLFW.");
        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        let gl = unsafe {
            glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _)
        };
        let version = gl.version();
        let title = format!("{} - {:?}", title, version);
        window.set_title(&title);

        unsafe {
            gl.viewport(0, 0, width as i32, height as i32);
            gl.clear_color(0.0, 0.0, 0.0, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
        }

        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);
        window.make_current();

        Self {
            title,
            width,
            height,
            resizable: false,
            vsync: false,
            fullscreen: false,
            cursor: true,
            cursor_grabbed: false,
            glfw,
            window,
            events,
            gl: Arc::new(gl),
        }
    }

    fn error_callback(err: glfw::Error, description: String) {
        panic!("GLFW error {:?}: {:?}", err, description);
    }
    pub(crate) fn input_handler(
        &self,
        event: glfw::WindowEvent,
        keys: &mut Keys,
        mouse: &mut Mouse,
    ) {
        match event {
            glfw::WindowEvent::Key(key, _, action, _) => {
                keys.set_key(key, action);
            }
            glfw::WindowEvent::MouseButton(button, action, _) => {
                mouse.set_mouse_button(button, action);
            }
            glfw::WindowEvent::CursorPos(x, y) => {
                mouse.set_mouse_pos(x, y);
            }
            glfw::WindowEvent::Scroll(x, y) => {
                mouse.set_scroll_offset(x, y);
            }
            _ => {}
        }
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
