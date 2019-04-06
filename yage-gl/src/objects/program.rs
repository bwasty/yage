use std::collections::HashMap;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::str;

use gl;
use gl::types::*;

// use cgmath::{Matrix, Matrix4, Vector3, Vector4};
// use cgmath::prelude::*;

// use log::{warn, trace};

use crate::{GL, GlFunctions};

// TODO!!!: copied from gltf-viewer (struct Shader) for debugging, barely adapted
// - native only (-> get rid of direct gl:: calls)
pub struct Program<'a> {
    pub id: u32,
    gl: &'a GL,
    uniform_location_cache: HashMap<&'static str, i32>
}

impl<'a> Program<'a> {
    #[allow(dead_code)]
    pub fn new(gl: &'a GL, vertex_path: &str, fragment_path: &str, defines: &[String]) -> Program<'a> {
        // 1. retrieve the vertex/fragment source code from filesystem
        let mut v_shader_file = File::open(vertex_path).unwrap_or_else(|_| panic!("Failed to open {}", vertex_path));
        let mut f_shader_file = File::open(fragment_path).unwrap_or_else(|_| panic!("Failed to open {}", fragment_path));
        let mut vertex_code = String::new();
        let mut fragment_code = String::new();
        v_shader_file
            .read_to_string(&mut vertex_code)
            .expect("Failed to read vertex shader");
        f_shader_file
            .read_to_string(&mut fragment_code)
            .expect("Failed to read fragment shader");

        Self::from_source(gl, &vertex_code, &fragment_code, defines)
    }

    // TODO!!: generic GL/ impl Trait?
    pub fn from_source(gl: &'a GL, vertex_code: &str, fragment_code: &str, defines: &[String]) -> Program<'a> {
        let mut program = Self {
            id: 0,
            gl,
            uniform_location_cache: HashMap::new()
        };

        let vertex_code = Self::add_defines(vertex_code, defines);
        let fragment_code = Self::add_defines(fragment_code, defines);

        // 2. compile shaders
        unsafe {
            // vertex shader
            let vertex = gl.create_shader(glenum::ShaderKind::Vertex);
            gl.shader_source(vertex, &vertex_code);
            gl.compile_shader(vertex);
            program.check_compile_errors(vertex, "VERTEX");
            // fragment Shader
            let fragment = gl.create_shader(glenum::ShaderKind::Fragment);
            gl.shader_source(fragment, &fragment_code);
            gl.compile_shader(fragment);
            program.check_compile_errors(fragment, "FRAGMENT");
            // shader Program
            let id = gl.create_program();
            gl.attach_shader(id, vertex);
            gl.attach_shader(id, fragment);
            gl.link_program(id);
            program.check_compile_errors(id, "PROGRAM");
            // delete the shaders as they're linked into our program now and no longer necessary
            gl.delete_shader(vertex);
            gl.delete_shader(fragment);
            program.id = id;
        }

        program
    }

    fn add_defines(source: &str, defines: &[String]) -> String {
        // insert preprocessor defines after #version if exists
        // (#version must occur before any other statement in the program)
        let defines = defines.iter()
            .map(|define| format!("#define {}", define))
            .collect::<Vec<_>>()
            .join("\n");
        let mut lines: Vec<_> = source.lines().collect();
        if let Some(version_line) = lines.iter().position(|l| l.starts_with("#version")) {
            lines.insert(version_line+1, &defines);
        }
        else {
            lines.insert(0, &defines);
        }
        lines.join("\n")
    }

    /// activate the shader
    /// ------------------------------------------------------------------------
    pub fn use_program(&self) {
        self.gl.use_program(Some(self.id))
    }

    /// utility uniform functions
    /// ------------------------------------------------------------------------
    #[allow(dead_code)]
    pub unsafe fn set_bool(&self, location: i32, value: bool) {
        gl::Uniform1i(location, value as i32);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_int(&self, location: i32, value: i32) {
        gl::Uniform1i(location, value);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_float(&self, location: i32, value: f32) {
        gl::Uniform1f(location, value);
    }
    /// ------------------------------------------------------------------------
    // pub unsafe fn set_vector3(&self, location: i32, value: &Vector3<f32>) {
    //     gl::Uniform3fv(location, 1, value.as_ptr());
    // }
    // /// ------------------------------------------------------------------------
    // pub unsafe fn set_vector4(&self, location: i32, value: &Vector4<f32>) {
    //     gl::Uniform4fv(location, 1, value.as_ptr());
    // }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_vec2(&self, location: i32, x: f32, y: f32) {
        gl::Uniform2f(location, x, y);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_vec3(&self, location: i32, x: f32, y: f32, z: f32) {
        gl::Uniform3f(location, x, y, z);
    }
    /// ------------------------------------------------------------------------
    // pub unsafe fn set_mat4(&self, location: i32, mat: &Matrix4<f32>) {
    //     gl::UniformMatrix4fv(location, 1, gl::FALSE, mat.as_ptr());
    // }

    /// get uniform location with caching
    pub unsafe fn uniform_location(&mut self, name: &'static str) -> i32 {
        if let Some(loc) = self.uniform_location_cache.get(name) {
            return *loc;
        }

        let c_name = CString::new(name).unwrap();
        let loc = gl::GetUniformLocation(self.id, c_name.as_ptr());
        if loc == -1 {
            // TODO!: trace!
            println!("uniform '{}' unknown for shader {}", name, self.id);
        }
        self.uniform_location_cache.insert(name, loc);
        loc
    }

    /// utility function for checking shader compilation/linking errors.
    /// ------------------------------------------------------------------------
    unsafe fn check_compile_errors(&self, shader: u32, type_: &str) {
        let mut success = i32::from(gl::FALSE);
        let mut info_log = Vec::with_capacity(1024);
        info_log.set_len(1024 - 1); // subtract 1 to skip the trailing null character
        if type_ != "PROGRAM" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            let log_type = if success == i32::from(gl::TRUE) { "WARNING" } else { "ERROR" };
            let mut length = 0;
            gl::GetShaderInfoLog(shader, 1024, &mut length, info_log.as_mut_ptr() as *mut GLchar);
            if length == 0 { return }
            panic!("{}::SHADER_COMPILATION_{} of type: {}\n{}",
                      log_type, log_type,
                      type_,
                      str::from_utf8(&info_log[0..length as usize]).unwrap());

        } else {
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            let log_type = if success == i32::from(gl::TRUE) { "WARNING" } else { "ERROR" };
            let mut length = 0;
            gl::GetProgramInfoLog(shader, 1024, &mut length, info_log.as_mut_ptr() as *mut GLchar);
            if length == 0 { return }
            // TODO!: warn!
            println!("{}::PROGRAM_LINKING_{} of type: {}\n{}",
                      log_type, log_type,
                      type_,
                      str::from_utf8(&info_log[0..length as usize]).unwrap());
        }

    }
}

