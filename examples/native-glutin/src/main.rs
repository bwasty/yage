use glutin::GlContext;

use yage::gl::{
    GL, GlFunctions,
    glenum,
    check_error,
    objects::{Program, Buffer, VertexArray}
};
use yage::glutin::{
    Application,
    Context,
    Window
};

fn main() {
    // create application
    let mut app = Application::new();

    // create window
    let window_id = app.add_window(Window::new(&app));

    // activate context
    app.window(window_id).unwrap().make_current();

    // resolve OpenGL functions
    // [TODO] automate this
    let gl_window = app.window(window_id).unwrap().get_gl_window();

    // create OpenGL wrapper
    let gl = GL::new();

    gl.clear_color(0.1, 0.2, 0.3, 1.0);

    let program = Program::from_source(&gl, VS_SRC, FS_SRC, &[]);
    program.use_program();

    let vertex_buffer = Buffer::new(&gl, glenum::BufferKind::Array as _);
    vertex_buffer.bind();
    vertex_buffer.set_data(&VERTEX_DATA, glenum::DrawMode::Static as _);

    let vao = VertexArray::new(&gl);
    vao.bind();

    vertex_buffer.attrib_enable(
        0,
        2,
        gl::FLOAT,
        false,
        5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
    0);

    vertex_buffer.attrib_enable(
        1,
        3,
        gl::FLOAT,
        false,
        5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
        2 * std::mem::size_of::<f32>() as i32,
    );

    check_error!();

    gl.viewport(0, 0, 300, 200);

    while app.is_running() {
        app.poll_events();

        gl.clear(glenum::BufferBit::Color as u32);
        gl.draw_arrays(gl::TRIANGLES, 0, 3);

        // check_error!();

        app.window(window_id).unwrap().swap_buffers();
    }
}


const VS_SRC: &str = "
#version 330 core
precision mediump float;
layout (location = 0) in vec2 position;
layout (location = 1) in vec3 color;
out vec3 v_color;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_color = color;
}";

const FS_SRC: &str = "
#version 330 core
precision mediump float;
in vec3 v_color;
out vec4 FragColor;
void main() {
    FragColor = vec4(v_color, 1.0);
}";

#[rustfmt::skip]
static VERTEX_DATA: [f32; 15] = [
    -0.5, -0.5,  1.0,  0.0,  0.0,
     0.0,  0.5,  0.0,  1.0,  0.0,
     0.5, -0.5,  0.0,  0.0,  1.0,
];
