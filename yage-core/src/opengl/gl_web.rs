use web_sys::{
    WebGl2RenderingContext, WebGlBuffer, WebGlFramebuffer, WebGlProgram, WebGlRenderbuffer,
    WebGlShader, WebGlTexture, WebGlUniformLocation, WebGlVertexArrayObject, WebGlTransformFeedback
};

use crate::opengl::glenum;

pub struct GL {
    // TODO: support WebGL1?
    gl: WebGl2RenderingContext,
}

impl GL {
    pub fn from_webgl_context(context: WebGl2RenderingContext) -> GL {
        GL { gl: context }
    }
}

impl super::GlFunctions for GL {
    type GlShader = WebGlShader;
    type GlProgram = WebGlProgram;
    type GlBuffer = WebGlBuffer;
    type GlVertexArray = WebGlVertexArrayObject;
    type GlTexture = WebGlTexture;
    type GlUniformLocation = WebGlUniformLocation;
    type GlFramebuffer = WebGlFramebuffer;
    type GlRenderbuffer = WebGlRenderbuffer;
    type GlTransformFeedback = WebGlTransformFeedback;

    // View and Clip

    fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        self.gl.viewport(x, y, width, height);
    }

    fn scissor(&self, x: i32, y: i32, width: i32, height: i32) {
        self.gl.scissor(x, y, width, height);
    }

    // Programs and Shaders

    fn create_shader(&self, kind: glenum::types::GLenum) -> Self::GlShader {
        self.gl.create_shader(kind).unwrap()
    }

    fn shader_source(&self, shader: &Self::GlShader, source: &str) {
        self.gl.shader_source(shader, source);
    }

    fn compile_shader(&self, shader: &Self::GlShader) {
        self.gl.compile_shader(shader);
    }

    fn delete_shader(&self, shader: &Self::GlShader) {
        self.gl.delete_shader(Some(shader));
    }

    fn get_shader_parameter(&self, shader: &Self::GlShader, param: u32) -> i32 {
        // TODO!!: multi-return type problem...try in cascade? (as_f64 fails for boolean case)
        self.gl
            .get_shader_parameter(shader, param)
            .as_bool()
            .unwrap() as i32
    }

    fn get_shader_info_log(&self, shader: &Self::GlShader) -> String {
        self.gl.get_shader_info_log(shader).unwrap()
    }

    fn create_program(&self) -> Self::GlProgram {
        self.gl.create_program().unwrap()
    }

    fn attach_shader(&self, program: &Self::GlProgram, shader: &Self::GlShader) {
        self.gl.attach_shader(&program, shader);
    }

    fn detach_shader(&self, program: &Self::GlProgram, shader: &Self::GlShader) {
        self.gl.detach_shader(&program, shader);
    }

    fn link_program(&self, program: &Self::GlProgram) {
        self.gl.link_program(&program);
    }

    fn get_program_parameter(&self, program: &Self::GlProgram, param: u32) -> i32 {
        // TODO!!: see get_shader_parameter....
        self.gl
            .get_program_parameter(&program, param)
            .as_bool()
            .unwrap() as i32
    }

    fn get_program_info_log(&self, program: &Self::GlProgram) -> String {
        self.gl.get_program_info_log(&program).unwrap()
    }

    fn use_program(&self, program: Option<&Self::GlProgram>) {
        self.gl.use_program(program);
    }

    fn get_attrib_location(&self, program: &Self::GlProgram, name: &str) -> i32 {
        self.gl.get_attrib_location(program, name)
    }

    fn bind_attrib_location(&self, program: &Self::GlProgram, index: u32, name: &str) {
        self.gl.bind_attrib_location(program, index, name);
    }

    fn delete_program(&self, program: &Self::GlProgram) {
        self.gl.delete_program(Some(program));
    }

    // Buffer Objects

    fn create_buffer(&self) -> Self::GlBuffer {
        self.gl.create_buffer().unwrap()
    }

    fn bind_buffer(&self, target: u32, buffer: Option<&Self::GlBuffer>) {
        self.gl.bind_buffer(target, buffer);
    }

    fn buffer_data<T>(&self, target: u32, data: &[T], usage: u32) {
        unsafe {
            self.gl.buffer_data_with_u8_array(
                target,
                std::slice::from_raw_parts(
                    data.as_ptr() as *const u8,
                    data.len() * std::mem::size_of::<T>(),
                ),
                usage,
            );
        }
    }

    fn buffer_sub_data<T>(&self, target: u32, offset: isize, data: &[T]) {
        unsafe {
            self.gl.buffer_sub_data_with_i32_and_u8_array(
                target,
                offset as i32,
                std::slice::from_raw_parts_mut(
                    data.as_ptr() as *mut u8,
                    data.len() * std::mem::size_of::<T>(),
                ),
            );
        }
    }

    fn delete_buffer(&self, buffer: &Self::GlBuffer) {
        self.gl.delete_buffer(Some(&buffer));
    }

    fn is_buffer(&self, buffer: &Self::GlBuffer) -> bool {
        self.gl.is_buffer(Some(buffer))
    }

    // Vertex Array Objects

    fn create_vertex_array(&self) -> Self::GlVertexArray {
        self.gl.create_vertex_array().unwrap()
    }

    fn bind_vertex_array(&self, vertex_array: Option<&Self::GlVertexArray>) {
        self.gl.bind_vertex_array(vertex_array)
    }

    fn delete_vertex_array(&self, vertex_array: &Self::GlVertexArray) {
        self.gl.delete_vertex_array(Some(vertex_array));
    }

    // Uniforms and Attributes

    fn vertex_attrib_pointer(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        normalized: bool,
        stride: i32,
        offset: i32,
    ) {
        self.gl
            .vertex_attrib_pointer_with_i32(index, size, data_type, normalized, stride, offset);
    }

    fn enable_vertex_attrib_array(&self, index: u32) {
        self.gl.enable_vertex_attrib_array(index);
    }

    fn disable_vertex_attrib_array(&self, index: u32) {
        self.gl.disable_vertex_attrib_array(index);
    }

    fn get_uniform_location(
        &self,
        program: &Self::GlProgram,
        name: &str,
    ) -> Self::GlUniformLocation {
        self.gl.get_uniform_location(program, name).unwrap()
    }

    fn uniform_1i(&self, location: &Self::GlUniformLocation, x: i32) {
        self.gl.uniform1i(Some(location), x);
    }

    fn uniform_1ui(&self, location: &Self::GlUniformLocation, x: u32) {
        self.gl.uniform1ui(Some(location), x);
    }

    fn uniform_1f(&self, location: &Self::GlUniformLocation, x: f32) {
        self.gl.uniform1f(Some(location), x);
    }

    fn uniform_2i(&self, location: &Self::GlUniformLocation, x: i32, y: i32) {
        self.gl.uniform2i(Some(location), x, y);
    }

    fn uniform_2ui(&self, location: &Self::GlUniformLocation, x: u32, y: u32) {
        self.gl.uniform2ui(Some(location), x, y);
    }

    fn uniform_2f(&self, location: &Self::GlUniformLocation, x: f32, y: f32) {
        self.gl.uniform2f(Some(location), x, y);
    }

    fn uniform_3i(&self, location: &Self::GlUniformLocation, x: i32, y: i32, z: i32) {
        self.gl.uniform3i(Some(location), x, y, z);
    }

    fn uniform_3ui(&self, location: &Self::GlUniformLocation, x: u32, y: u32, z: u32) {
        self.gl.uniform3ui(Some(location), x, y, z);
    }

    fn uniform_3f(&self, location: &Self::GlUniformLocation, x: f32, y: f32, z: f32) {
        self.gl.uniform3f(Some(location), x, y, z);
    }

    fn uniform_4i(&self, location: &Self::GlUniformLocation, x: i32, y: i32, z: i32, w: i32) {
        self.gl.uniform4i(Some(location), x, y, z, w);
    }

    fn uniform_4ui(&self, location: &Self::GlUniformLocation, x: u32, y: u32, z: u32, w: u32) {
        self.gl.uniform4ui(Some(location), x, y, z, w);
    }

    fn uniform_4f(&self, location: &Self::GlUniformLocation, x: f32, y: f32, z: f32, w: f32) {
        self.gl.uniform4f(Some(location), x, y, z, w);
    }

    fn uniform_matrix_2fv(&self, _location: &Self::GlUniformLocation, _mat: &[[f32; 2]; 2]) {
        // TODO!!: how to convert properly?
        // self.gl.uniform_matrix4fv_with_f32_array(Some(location), false, std::mem::transmute(mat));
        unimplemented!();
    }

    fn uniform_matrix_3fv(&self, _location: &Self::GlUniformLocation, _mat: &[[f32; 3]; 3]) {
        // TODO!!: how to convert properly?
        // self.gl.uniform_matrix4fv_with_f32_array(Some(location), false, std::mem::transmute(mat));
        unimplemented!();
    }

    fn uniform_matrix_4fv(&self, _location: &Self::GlUniformLocation, _mat: &[[f32; 4]; 4]) {
        // TODO!!: how to convert properly?
        // self.gl.uniform_matrix4fv_with_f32_array(Some(location), false, std::mem::transmute(mat));
        unimplemented!();
    }

    // Writing to the Draw Buffer

    fn draw_arrays(&self, mode: u32, first: i32, count: i32) {
        self.gl.draw_arrays(mode, first, count);
    }

    fn draw_elements(&self, mode: u32, count: i32, element_type: u32, offset: i32) {
        self.gl
            .draw_elements_with_i32(mode, count, element_type, offset);
    }

    fn vertex_attrib_divisor(&self, index: u32, divisor: u32) {
        self.gl.vertex_attrib_divisor(index, divisor);
    }

    fn draw_arrays_instanced(&self, mode: u32, first: i32, count: i32, instance_count: i32) {
        self.gl.draw_arrays_instanced(mode, first, count, instance_count);
    }

    fn draw_elements_instanced(
        &self,
        mode: u32,
        count: i32,
        element_type: u32,
        offset: i32,
        instance_count: i32,
    ) {
        self.gl.draw_elements_instanced_with_i32(
            mode as u32,
            count,
            element_type as u32,
            offset,
            instance_count,
        );
    }

    // Special Functions

    fn enable(&self, param: u32) {
        self.gl.enable(param);
    }

    fn disable(&self, param: u32) {
        self.gl.disable(param);
    }

    fn finish(&self) {
       self.gl.finish();
    }

    fn flush(&self) {
        self.gl.flush();
    }

    fn get_error(&self) -> u32 {
        self.gl.get_error()
    }

    fn get_parameter_i32(&self, parameter: u32) -> i32 {
        self.gl.get_parameter(parameter)
            .unwrap()
            .as_f64()
            .unwrap() as i32
    }

    fn pixel_storei(&self, storage: u32, value: i32) {
        self.gl.pixel_storei(storage, value);
    }

    /// Unimplemented - method missing in WebGL (and ES2, ES3)
    fn point_size(&self, _size: f32) {
        // TODO!: log warning instead of panic?
        unimplemented!("method not available in WebGL")
    }

    // Texture Objects

    fn active_texture(&self, unit: u32) {
        self.gl.active_texture(unit);
    }

    fn bind_texture(&self, target: u32, texture: Option<&Self::GlTexture>) {
        self.gl.bind_texture(target, texture);
    }

    fn create_texture(&self) -> Self::GlTexture {
        self.gl.create_texture().unwrap()
    }

    fn delete_texture(&self, texture: &Self::GlTexture) {
        self.gl.delete_texture(Some(texture));
    }

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
    ) {
        // TODO!: unused_must_use - return Result?
        let _ = self
            .gl
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                target,
                level,
                internal_format,
                width,
                height,
                border,
                format,
                ty,
                pixels,
            );
    }

    fn tex_image_3d(
        &self,
        target: u32,
        level: i32,
        internal_format: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        ty: u32,
        pixels: Option<&[u8]>,
    ) {
        // TODO!: unused_must_use - return Result?
        let _ = self
            .gl
            .tex_image_3d_with_u8_array_and_src_offset(
                target,
                level,
                internal_format,
                width,
                height,
                depth,
                border,
                format,
                ty,
                pixels.unwrap(), // TODO!: none case?
                0
            );
    }

    fn generate_mipmap(&self, target: u32) {
        self.gl.generate_mipmap(target);
    }

    fn tex_parameteri(&self, target: u32, parameter: u32, value: i32) {
        self.gl.tex_parameteri(target, parameter, value);
    }

    fn is_texture(&self, texture: &Self::GlTexture) -> bool {
        self.gl.is_texture(Some(texture))
    }

    fn create_framebuffer(&self) -> Self::GlFramebuffer {
        self.gl.create_framebuffer().unwrap()
    }

    fn delete_framebuffer(&self, framebuffer: &Self::GlFramebuffer) {
        self.gl.delete_framebuffer(Some(framebuffer));
    }

    fn bind_framebuffer(&self, target: u32, framebuffer: Option<&Self::GlFramebuffer>) {
        self.gl.bind_framebuffer(target, framebuffer);
    }

    fn framebuffer_texture_2d(
        &self,
        target: u32,
        attachment: u32,
        texture_target: u32,
        texture: Option<&Self::GlTexture>,
        level: i32,
    ) {
        self.gl
            .framebuffer_texture_2d(target, attachment, texture_target, texture, level);
    }

    fn framebuffer_renderbuffer(
        &self,
        target: u32,
        attachment: u32,
        renderbuffer_target: u32,
        renderbuffer: Option<&Self::GlRenderbuffer>,
    ) {
        self.gl
            .framebuffer_renderbuffer(target, attachment, renderbuffer_target, renderbuffer);
    }

    fn is_frambuffer(&self, framebuffer: &Self::GlFramebuffer) -> bool {
        self.gl.is_framebuffer(Some(framebuffer))
    }

    fn check_framebuffer_status(&self, target: u32) -> u32 {
        self.gl.check_framebuffer_status(target)
    }

    fn blit_framebuffer(
        &self,
        src_x0: i32,
        src_y0: i32,
        src_x1: i32,
        src_y1: i32,
        dst_x0: i32,
        dst_y0: i32,
        dst_x1: i32,
        dst_y1: i32,
        mask: u32,
        filter: u32,
    ) {
        self.gl.blit_framebuffer(
            src_x0, src_y0, src_x1, src_y1, dst_x0, dst_y0, dst_x1, dst_y1, mask, filter
        );
    }

    fn read_buffer(&self, mode: u32) {
        self.gl.read_buffer(mode);
    }

    // Renderbuffer Objects

    fn create_renderbuffer(&self) -> Self::GlRenderbuffer {
        self.gl.create_renderbuffer().unwrap()
    }

    fn delete_renderbuffer(&self, renderbuffer: &Self::GlRenderbuffer) {
        self.gl.delete_renderbuffer(Some(renderbuffer));
    }

    fn bind_renderbuffer(&self, target: u32, renderbuffer: Option<&Self::GlRenderbuffer>) {
        self.gl.bind_renderbuffer(target, renderbuffer);
    }

    fn renderbuffer_storage(&self, target: u32, internal_format: u32, width: i32, height: i32) {
        self.gl
            .renderbuffer_storage(target, internal_format, width, height);
    }

    // Per-Fragment Operations

    fn depth_func(&self, func: u32) {
        self.gl.depth_func(func);
    }

    fn blend_func(&self, src: u32, dst: u32) {
        self.gl.blend_func(src, dst);
    }

    fn blend_func_separate(&self, src_rgb: u32, dst_rgb: u32, src_alpha: u32, dst_alpha: u32) {
        self.gl.blend_func_separate(src_rgb, dst_rgb, src_alpha, dst_alpha);
    }

    fn stencil_func(&self, func: u32, reference: i32, mask: u32){
        self.gl.stencil_func(func, reference, mask);
    }

    fn stencil_op(&self, stencil_fail: u32, depth_fail: u32, pass: u32){
        self.gl.stencil_op(stencil_fail, depth_fail, pass);
    }

    // Whole Framebuffer Operations

    fn clear(&self, mask: u32) {
        self.gl.clear(mask);
    }

    fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        self.gl.clear_color(r, g, b, a);
    }

    fn clear_depth(&self, depth: f32) {
        self.gl.clear_depth(depth);
    }

    fn clear_stencil(&self, stencil: i32) {
        self.gl.clear_stencil(stencil);
    }

    fn stencil_mask(&self, mask: u32){
        self.gl.stencil_mask(mask);
    }

    fn depth_mask(&self, value: bool) {
        self.gl.depth_mask(value);
    }

    // Read Back Pixels

    fn read_pixels(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: u32,
        type_: u32,
        data: &mut [u8],
    ) {
        // TODO: unused_must_use - return Result?
        let _ = self.gl.read_pixels_with_opt_u8_array(x, y, width, height,
            format, type_, Some(data));
    }

    // Rasterization

    fn cull_face(&self, value: u32) {
        self.gl.cull_face(value);
    }

    /// Unimplemented - method missing in WebGL (and ES2, ES3)
    fn polygon_mode(&self, _face: u32, _mode: u32) {
        // TODO!: log warning instead of panic?
        unimplemented!("method not available in WebGL")
    }

    // Multiple Render Targets

    /// unimplemented (yet)
    fn draw_buffers(&self, _buffers: &[u32]) {
        // TODO!!: requires JsValue...?
        // self.gl.draw_buffers(buffers);
        unimplemented!()
    }

    // Transform Feedback

    // fn create_transform_feedback(&self) -> Self::GlTransformFeedback {
    //     self.gl.create_transform_feedback().unwrap()
    // }
}
