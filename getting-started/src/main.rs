extern crate graphics;
extern crate piston;
extern crate sdl2_game_window;
extern crate opengl_graphics;
extern crate shader_version;

use sdl2_game_window::WindowSDL2;
use opengl_graphics::Gl;
use shader_version::opengl::OpenGL_2_1;

use piston::{
    Window,
    Render,
    RenderArgs,
    Update,
    UpdateArgs
};

use graphics::{
    Context,
    AddRectangle,
    AddColor,
    Draw,
    RelativeTransform2d,
};

pub struct App {
    gl: Gl,       // OpenGL drawing backend.
    rotation: f64 // Rotation for the square.
}

impl<W: Window> App {
    fn render(&mut self, _: &mut W, args: &RenderArgs) {
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

    fn update(&mut self, _: &mut W, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    // Create an SDL window.
    let mut window = WindowSDL2::new(
        piston::shader_version::opengl::OpenGL_2_1,
        piston::WindowSettings::default()
    );

    // Some settings for how the game should be run.
    let event_settings = piston::EventSettings {
        updates_per_second: 60,
        max_frames_per_second: 60
    };

    // Create a new game and run it.
    let mut app = App { gl: Gl::new(OpenGL_2_1), rotation: 0.0 };

    for e in piston::EventIterator::new(&mut window, &event_settings) {
        match e {
            Render(_args) =>
                app.render(&mut window, &_args),
            Update(_args) =>
                app.update(&mut window, &_args),
            _ => {},
        }
    }
}
