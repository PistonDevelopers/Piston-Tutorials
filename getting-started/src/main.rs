extern crate graphics;
extern crate piston;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate shader_version;
extern crate event;

use sdl2_window::Sdl2Window;
use opengl_graphics::Gl;
use shader_version::opengl::OpenGL::OpenGL_3_2;

use std::cell::RefCell;
use piston::{
    RenderArgs,
    UpdateArgs
};

use graphics::{
    Context,
    AddRectangle,
    AddColor,
    Draw,
    RelativeTransform,
};

use event::{
    Events,
    Window,
    RenderEvent,
    UpdateEvent,
};

pub struct App {
    gl: Gl,       // OpenGL drawing backend.
    rotation: f64 // Rotation for the square.
}

impl App {
    fn render<W: Window>(&mut self, _: &mut W, args: &RenderArgs) {
        // Set up a context to draw into.
        let context = &Context::abs(args.width as f64, args.height as f64);
        // Clear the screen.
        context.rgba(0.0,1.0,0.0,1.0).draw(&mut self.gl);

        // Draw a box rotating around the middle of the screen.
        context
            .trans((args.width / 2) as f64, (args.height / 2) as f64)
            .rot_rad(self.rotation)
            .rect(0.0, 0.0, 50.0, 50.0)
            .rgba(1.0, 0.0, 0.0,1.0)
            .trans(-25.0, -25.0)
            .draw(&mut self.gl);
    }

    fn update<W: Window>(&mut self, _: &mut W, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    // Create an SDL window.
    let window = Sdl2Window::new(
        OpenGL_3_2,
        piston::WindowSettings::default()
    );

    // Create a new game and run it.
    let mut app = App { gl: Gl::new(OpenGL_3_2), rotation: 0.0 };

    let window = RefCell::new(window);
    for e in Events::new(&window) {
        e.render(|r| app.render(window.borrow_mut().deref_mut(), r));
        e.update(|u| app.update(window.borrow_mut().deref_mut(), u));
    }
}
