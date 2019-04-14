use std::rc::Rc;

use crate::{GlFunctions, GL};

/// Wrapper around an OpenGL vertex array.
pub struct VertexArray {
    gl: Rc<GL>,
    array_handle: <GL as GlFunctions>::GlVertexArray,
}

impl VertexArray {
    /// Creates a vertex array.
    ///
    /// # Parameters
    /// - `gl`: GL context
    pub fn new(gl: &Rc<GL>) -> Self {
        Self {
            gl: gl.clone(),
            array_handle: gl.create_vertex_array(),
        }
    }

    /// Getter for the OpenGL/WebGL handle
    pub fn handle(&self) -> &<GL as GlFunctions>::GlVertexArray {
        &self.array_handle
    }

    /// Binds the vertex array.
    pub fn bind(&self) {
        self.gl.bind_vertex_array(Some(&self.array_handle));
    }

    /// Unbinds the vertex array.
    pub fn unbind(&self) {
        self.gl.bind_vertex_array(None);
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        self.gl.delete_vertex_array(&self.array_handle);
    }
}
