extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

use piston::window::{ WindowSettings, Size };
use piston::event::*;
use sdl2_window::Sdl2Window as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64   // Rotation for the square.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        // Set up a context to draw into.
        let context = &Context::abs(args.width as f64, args.height as f64);

        let center_context = &context.trans((args.width / 2) as f64, (args.height / 2) as f64)
                                     .rot_rad(self.rotation)
                                     .trans(-25.0, -25.0);
        let square = rectangle::square(0.0, 0.0, 50.0);

        self.gl.draw([0, 0, args.width as i32, args.height as i32], |_, gl| {
            // Clear the screen.
            clear(GREEN, gl);
            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, center_context.transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    let opengl = OpenGL::_3_2;

    // Create an SDL window.
    let window = Window::new(
        opengl,
        WindowSettings::new("spinning-square".to_string(),
                            Size { width: 200, height: 200 })
                           .exit_on_esc(true)
    );

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0
    };

    for e in window.events() {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
