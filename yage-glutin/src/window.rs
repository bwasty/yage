use glutin::GlContext;
use glutin::WindowId;

use crate::Application;
use crate::Context;

///
/// Top-level window with OpenGL context.
///
pub struct Window {
    window: glutin::GlWindow,
}

impl Window {
    ///
    /// Create a new window for an application
    ///
    /// This creates a new top-level window using the events loop of the
    /// specified application. Afterwards, you should move the window
    /// into the application using Application::add_window().
    ///
    /// # Parameters
    /// - `application`: Application for which the window is created.
    ///
    /// # Returns
    /// A new instance of Window.
    ///
    pub fn new(application: &Application) -> Window {
        // create window builder
        let window_builder = glutin::WindowBuilder::new()
            .with_title("A fantastic window!")
            .with_dimensions(glutin::dpi::LogicalSize::new(300.0, 200.0));

        // create context builder
        let context_builder = glutin::ContextBuilder::new();

        // create actual OpenGL window
        let gl_window =
            glutin::GlWindow::new(window_builder, context_builder, application.events_loop())
                .unwrap();

        gl::load_with(|ptr| gl_window.context().get_proc_address(ptr) as *const _);

        // create window
        Window { window: gl_window }
    }

    ///
    /// Get window ID
    ///
    /// # Returns
    /// The ID of the window.
    ///
    pub fn id(&self) -> WindowId {
        self.window.id()
    }

    ///
    /// Borrow reference to the wrapped OpenGL window
    ///
    /// # Returns
    /// Reference to the OpenGL window.
    ///
    pub fn get_gl_window(&self) -> &glutin::GlWindow {
        &self.window
    }

    ///
    /// Swap OpenGL buffers
    ///
    pub fn swap_buffers(&self) {
        let _ = self.window.swap_buffers();
    }

    ///
    /// Set window title
    ///
    /// # Parameters
    /// - `title`: The new window title
    ///
    pub fn set_title(&self, title: &str) {
        self.window.set_title(title);
    }
}

impl Context for Window {
    fn make_current(&self) {
        let _ = unsafe { self.window.make_current() };
    }
}
