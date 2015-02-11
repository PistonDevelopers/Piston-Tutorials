#![feature(core)]

extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

use std::cell::RefCell;
use piston::window::WindowSettings;
use piston::event::{
    events,
    RenderArgs,
    RenderEvent,
    UpdateArgs,
    UpdateEvent
};
use graphics::{
    Context,
    Rectangle,
    RelativeTransform
};
use sdl2_window::Sdl2Window as Window;
use opengl_graphics::{ Gl, OpenGL };

pub struct App {
    gl: Gl,       // OpenGL drawing backend.
    rotation: f64 // Rotation for the square.
}

impl App {
    fn render(&mut self, _: &mut Window, args: &RenderArgs) {
        const GREEN:  [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:    [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const SQUARE: [f64; 4] = [0.0, 0.0, 50.0, 50.0];

        // Set up a context to draw into.
        let context = &Context::abs(args.width as f64, args.height as f64);

        // Clear the screen.
        graphics::clear(GREEN, &mut self.gl);

        // Draw a box rotating around the middle of the screen.
        let center_context = &context.trans((args.width / 2) as f64, (args.height / 2) as f64)
                                     .rot_rad(self.rotation)
                                     .trans(-25.0, -25.0);
        Rectangle::new(RED).draw(SQUARE, center_context, &mut self.gl);
    }

    fn update(&mut self, _: &mut Window, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    // Create an SDL window.
    let window = Window::new(
        OpenGL::_3_2,
        WindowSettings::default()
    );
    let window = RefCell::new(window);

    // Create a new game and run it.
    let mut app = App {
        gl: Gl::new(OpenGL::_3_2),
        rotation: 0.0
    };

    for e in events(&window) {
        if let Some(r) = e.render_args() {
            app.render(&mut window.borrow_mut(), &r);
        }

        if let Some(u) = e.update_args() {
            app.update(&mut window.borrow_mut(), &u);
        }
    }
}
