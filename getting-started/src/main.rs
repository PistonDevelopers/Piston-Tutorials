extern crate graphics;
extern crate piston;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate shader_version;
extern crate event;

use sdl2_window::Sdl2Window as Window;
use opengl_graphics::Gl;
use shader_version::opengl::OpenGL::_3_2;

use std::cell::RefCell;
use piston::{
    RenderArgs,
    UpdateArgs
};

use graphics::{
    Context,
    Rectangle,
    RelativeTransform,
};

use event::{
    RenderEvent,
    UpdateEvent,
};

pub struct App {
    gl: Gl,       // OpenGL drawing backend.
    rotation: f64 // Rotation for the square.
}

impl App {
    fn render(&mut self, _: &mut Window, args: &RenderArgs) {
        // Set up a context to draw into.
        let context = &Context::abs(args.width as f64, args.height as f64);
        // Clear the screen
        graphics::clear([0.0,1.0,0.0,1.0], &mut self.gl);

        // Draw a box rotating around the middle of the screen.
        let center_context = &context.trans((args.width / 2) as f64, (args.height / 2) as f64)
            .rot_rad(self.rotation)
            .trans(-25.0, -25.0);
        Rectangle::new([1.0, 0.0, 0.0, 1.0]).draw([0.0, 0.0, 50.0, 50.0], center_context, &mut self.gl);
    }

    fn update(&mut self, _: &mut Window, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    // Create an SDL window.
    let window = Window::new(
        _3_2,
        piston::WindowSettings::default()
            );

    // Create a new game and run it.
    let mut app = App { gl: Gl::new(_3_2), rotation: 0.0 };

    let window = RefCell::new(window);
    for e in event::events(&window) {
        if let Some(r) = e.render_args() {
            app.render(&mut *window.borrow_mut(), &r);
        }
        if let Some(u) = e.update_args() {
            app.update(&mut *window.borrow_mut(), &u);
        }
    }
}
