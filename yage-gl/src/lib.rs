#[cfg(not(target_arch = "wasm32"))]
#[path = "gl_native.rs"]
mod gl;

#[cfg(target_arch = "wasm32")]
#[path = "gl_web.rs"]
pub mod gl;

pub use crate::gl::*;

pub mod objects;

#[macro_use]
pub mod utils;
pub use utils::*;

pub use glenum;

use glenum::BufferBit;

/// Trait for all GL functions
/// Associated types are used to support different handles types in native GL and WebGL
/// (integers vs opaque JS types like `WebGLShader`)
/// Please refer to the OpenGL/WebGL documentation for details about each
/// function (hint: http://docs.gl/).
/// Some functions are named differently in WebGL than in OpenGL - we use the WebGL nomenclature
/// since they are clearer (e.g. create_buffer for gl::GenBuffers, 
/// get_shader_parameter for gl::GetShaderiv)
pub trait GlFunctions {
    type GlShader;
    type GlProgram;
    type GlBuffer;
    type GlVertexArray;
    type GlTexture;
    type GlUniformLocation;

    fn clear_color(&self, r: f32, g: f32, b: f32, a: f32);
    fn clear(&self, bit: BufferBit);

    fn viewport(&self, x: i32, y: i32, width: i32, height: i32);

    fn create_shader(&self, kind: glenum::ShaderKind) -> Self::GlShader;
    fn shader_source(&self, shader: &Self::GlShader, source: &str);
    fn compile_shader(&self, shader: &Self::GlShader);
    fn delete_shader(&self, shader: &Self::GlShader);
    /// Named after the WebGL function. See `gl::GetShaderiv` for OpenGL.
    fn get_shader_parameter(&self, shader: &Self::GlShader, param: u32) -> i32;
    fn get_shader_info_log(&self, shader: &Self::GlShader) -> String;

    fn create_program(&self) -> Self::GlProgram;
    fn attach_shader(&self, program: &Self::GlProgram, shader: &Self::GlShader);
    fn link_program(&self, program: &Self::GlProgram);
    fn get_program_parameter(&self, program: &Self::GlProgram, param: u32) -> i32;
    fn get_program_info_log(&self, program: &Self::GlProgram) -> String;
    fn use_program(&self, program: Option<&Self::GlProgram>);

    /// Named after the WebGL function. See `gl::GenBuffers` for OpenGL.
    fn create_buffer(&self) -> Self::GlBuffer;
    fn bind_buffer(&self, target: u32, buffer: Option<&Self::GlBuffer>);
    fn buffer_data<T>(&self, target: u32, data: &[T], usage: u32);

    fn create_vertex_array(&self) -> Self::GlVertexArray;
    fn bind_vertex_array(&self, vertex_array: Option<&Self::GlVertexArray>);
    fn vertex_attrib_pointer(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        normalized: bool,
        stride: i32,
        offset: i32,
    );
    fn enable_vertex_attrib_array(&self, index: u32);

    fn draw_arrays(&self, mode: u32, first: i32, count: i32);
    fn draw_elements(&self, mode: u32, count: i32, element_type: u32, offset: i32);

    fn enable(&self, param: u32);
    fn disable(&self, param: u32);

    fn point_size(&self, size: f32);

    fn active_texture(&self, unit: u32);
    fn bind_texture(&self, target: u32, texture: Option<&Self::GlTexture>);

    fn blend_func(&self, src: u32, dst: u32);

    fn create_texture(&self) -> Self::GlTexture;

    #[allow(clippy::too_many_arguments)]
    fn tex_image_2d(
        &self,
        target: u32,
        level: i32,
        internal_format: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        ty: u32,
        pixels: Option<&[u8]>,
    );

    fn generate_mipmap(&self);

    fn tex_parameteri(&self, target: u32, parameter: u32, value: i32);

    fn get_uniform_location(
        &self,
        program: &Self::GlProgram,
        name: &str,
    ) -> Self::GlUniformLocation;

    fn uniform_1i(&self, location: &Self::GlUniformLocation, x: i32);
    fn uniform_1f(&self, location: &Self::GlUniformLocation, x: f32);
    fn uniform_3fv(&self, location: &Self::GlUniformLocation, x: &[f32; 3]);
    fn uniform_4fv(&self, location: &Self::GlUniformLocation, x: &[f32; 4]);
    fn uniform_2f(&self, location: &Self::GlUniformLocation, x: f32, y: f32);
    fn uniform_3f(&self, location: &Self::GlUniformLocation, x: f32, y: f32, z: f32);
    fn uniform_matrix_4fv(&self, location: &Self::GlUniformLocation, value: &[[f32; 4]; 4]);

    // TODO!: "optional" methods from gltf-viewer (mostly for screenshots, headless):
    // framebuffer
    // viewer - polygonmode, pixelstorei, readpixels 
}
