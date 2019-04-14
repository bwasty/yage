use std::rc::Rc;

use crate::{GlFunctions, GL};

/// Wrapper around an OpenGL array or element array buffer.
pub struct Buffer {
    gl: Rc<GL>,
    /// Target for use in `glBindBuffer`
    target: u32,
    handle: <GL as GlFunctions>::GlBuffer,
}

impl Buffer {
    /// Creates an empty buffer.
    ///
    /// # Parameters
    /// - `gl`: GL context
    /// - `target`: must be a valid glenum for `glBindBuffer`
    pub fn new(gl: &Rc<GL>, target: u32) -> Self {
        Self {
            gl: gl.clone(),
            target,
            handle: gl.create_buffer(),
        }
    }

    /// Getter for the OpenGL/WebGL handle
    pub fn handle(&self) -> &<GL as GlFunctions>::GlBuffer {
        &self.handle
    }

    /// Creates the buffer object's data store.
    ///
    /// Expects the buffer to be bound.
    ///
    /// # Parameters
    /// - `data`: buffer data
    /// - `usage`: must be a valid glenum for `glBufferData`
    pub fn set_data<T>(&self, data: &[T], usage: u32) {
        self.gl.buffer_data(self.target, data, usage);
    }

    /// Updates a subset of a buffer object's data store.
    ///
    /// Expects the buffer to be bound.
    ///
    /// # Parameters
    /// - `offset`: offset into the buffer object's data store in bytes
    /// - `data`: buffer data
    pub fn set_sub_data<T>(&self, offset: isize, data: &[T]) {
        self.gl.buffer_sub_data(self.target, offset, data);
    }

    /// Binds the buffer.
    pub fn bind(&self) {
        self.gl.bind_buffer(self.target, Some(&self.handle));
    }

    /// Unbinds the buffer.
    pub fn unbind(&self) {
        self.gl.bind_buffer(self.target, None);
    }

    /// Specifies the memory layout of the buffer for a binding point.
    ///
    /// Expects the buffer to be bound.
    ///
    /// # Parameters
    /// - `index` - Index of the vertex attribute that is to be setup and enabled.
    /// - `size` - Number of components per vertex attribute.
    /// - `type` - Data type of each component in the array.
    /// - `normalized` - Whether integer data values should be normalized when being casted to a float.
    /// - `stride` - Offset in bytes between the beginning of consecutive vertex attributes.
    /// - `offset` - Offset in bytes of the first component in the vertex attribute array.
    pub fn attrib_enable(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        normalized: bool,
        stride: i32,
        offset: i32,
    ) {
        self.gl
            .vertex_attrib_pointer(index, size, data_type, normalized, stride, offset);
        self.gl.enable_vertex_attrib_array(index);
    }

    /// Disables a buffer binding point.
    /// - `index` - Index of the vertex attribute that is to be disabled.
    pub fn attrib_disable(&self, index: u32) {
        self.gl.disable_vertex_attrib_array(index);
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        self.gl.delete_buffer(&self.handle);
    }
}
