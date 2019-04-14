use glutin::GlContext;

use yage::gl::{
    check_error, glenum,
    objects::{Buffer, Program, VertexArray},
    GlFunctions, GL,
};

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_dimensions(glutin::dpi::LogicalSize::new(300.0, 200.0));
    let context = glutin::ContextBuilder::new();
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    let _ = unsafe { gl_window.make_current() };

    gl::load_with(|ptr| gl_window.context().get_proc_address(ptr) as *const _);

    let gl = GL::new();
    gl.clear_color(0.1, 0.2, 0.3, 1.0);

    let program = Program::from_source(&gl, VS_SRC, FS_SRC, &[]);
    program.use_program();

    let vao = VertexArray::new(&gl);
    vao.bind();

    let vertex_buffer = Buffer::new(&gl, glenum::BufferKind::Array as _);
    vertex_buffer
        .update()
        .set_data(&VERTEX_DATA, glenum::DrawMode::Static as _)
        .attrib_enable(
            0,
            2,
            gl::FLOAT,
            false,
            5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
            0,
        )
        .attrib_enable(
            1,
            3,
            gl::FLOAT,
            false,
            5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
            2 * std::mem::size_of::<f32>() as i32,
        );

    check_error!();

    gl.viewport(0, 0, 300, 200);

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            // println!("{:?}", event);
            #[allow(clippy::single_match)]
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(logical_size) => {
                        let dpi_factor = gl_window.get_hidpi_factor();
                        gl_window.resize(logical_size.to_physical(dpi_factor));
                    }
                    _ => (),
                },
                _ => (),
            }
        });

        gl.clear(glenum::BufferBit::Color);
        gl.draw_arrays(gl::TRIANGLES, 0, 3);

        // check_error!();

        let _ = gl_window.swap_buffers();
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
