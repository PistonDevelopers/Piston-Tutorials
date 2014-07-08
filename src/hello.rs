extern crate graphics;
extern crate piston;
extern crate glfw_game_window;
extern crate opengl_graphics;

use Window = glfw_game_window::GameWindowGLFW;

use piston::Game;
use piston::GameWindowSettings;
use piston::GameIteratorSettings;
use piston::RenderArgs;

use opengl_graphics::Gl;

use graphics::Context;
use graphics::add::AddRectangle;
use graphics::Draw;
use graphics::Draw;

pub struct App {
    gl: Gl,
    rotation: f64
}

impl Game for App {
    fn render(&mut self, args: &RenderArgs) {
        let context = &Context::abs(args.width as f64, args.height as f64);

        context
           // .rot_rad(self.rotation)
            .rect(0.0, 0.0, 5.0, 5.0)
            .rgb(1.0, 0.0, 0.0)
            .draw(self.gl);

        let dt = args.ext_dt;
        self.rotation += dt;
    }
}

fn main() {
    let mut window = Window::new(
        GameWindowSettings {
            title: "Hello Piston".to_string(),
            size: [800, 800],
            fullscreen: false,
            exit_on_esc: true
        }
    );

    let game_iter_settings = GameIteratorSettings {
        updates_per_second: 120,
        max_frames_per_second: 60
    };

    let mut app = App { gl: Gl::new(), rotation: 0.0 };

    app.run(&mut window, &game_iter_settings);
}
