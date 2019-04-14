use crate::{GlFunctions, GL};

/// Wrapper around an OpenGL array or element array buffer.
pub struct Buffer<'a> {
    gl: &'a GL,
    /// Target for use in `glBindBuffer`
    target: u32,
    buffer_handle: <GL as GlFunctions>::GlBuffer,
}

impl<'a> Buffer<'a> {
    /// Creates an empty buffer.
    ///
    /// # Parameters
    /// - `gl`: GL context
    /// - `target`: must be a valid glenum for `glBindBuffer`
    pub fn new(gl: &'a GL, target: u32) -> Self {
        Self {
            gl,
            target,
            buffer_handle: gl.create_buffer(),
        }
    }

    /// Binds the buffer.
    pub fn bind(&self) {
        self.gl.bind_buffer(self.target, Some(&self.buffer_handle));
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

    /// Bind the buffer and return BufferUpdater that can be used for updating
    ///
    /// # Example
    /// ```ignore
    /// let buffer = Buffer::new(gl);
    /// buffer.update()
    ///     .set_data(...)
    ///     .attrib_enable(...)
    /// ```
    ///
    /// Caution: assumes that you do nothing between calls to the updater that would
    /// unbind it!
    pub fn update(&self) -> BufferUpdater {
        // TODO!: boolean param for bind also here? (-> without would be simpler)
        BufferUpdater::new(self, true)
    }
}

impl<'a> Drop for Buffer<'a> {
    fn drop(&mut self) {
        self.gl.delete_buffer(&self.buffer_handle);
    }
}

pub struct BufferUpdater<'a> {
    buffer: &'a Buffer<'a>,
}

/// Builder-like object for updating without multiple binds.
impl<'a> BufferUpdater<'a> {
    /// It is expected that this is usually only constructed via Buffer::update().
    /// For exceptional cases where binding is not desired (already bound),
    /// this constructor can be used with `bind` set to `false`.
    pub fn new(buffer: &'a Buffer, bind: bool) -> Self {
        if bind {
            buffer.bind()
        }
        Self { buffer }
    }

    // TODO!!: self vs &self return types??

    /// Creates the buffer object's data store.
    ///
    /// Expects the buffer to be bound.
    ///
    /// # Parameters
    /// - `data`: buffer data
    /// - `usage`: must be a valid glenum for `glBufferData`
    pub fn set_data<T>(&self, data: &[T], usage: u32) -> &Self {
        self.buffer.gl.buffer_data(self.buffer.target, data, usage);
        self
    }

    /// Updates a subset of a buffer object's data store.
    ///
    /// Expects the buffer to be bound.
    ///
    /// # Parameters
    /// - `offset`: offset into the buffer object's data store in bytes
    /// - `data`: buffer data
    pub fn set_sub_data<T>(&self, offset: isize, data: &[T]) -> &Self {
        self.buffer
            .gl
            .buffer_sub_data(self.buffer.target, offset, data);
        self
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
    ) -> &Self {
        self.buffer
            .gl
            .vertex_attrib_pointer(index, size, data_type, normalized, stride, offset);
        self.buffer.gl.enable_vertex_attrib_array(index);
        self
    }

    /// Disables a buffer binding point.
    /// - `index` - Index of the vertex attribute that is to be disabled.
    pub fn attrib_disable(&self, index: u32) -> &Self {
        self.buffer.gl.disable_vertex_attrib_array(index);
        self
    }
}
