#![crate_name = "piston_roguelike"]
//! A Roguelike Game using Piston Engine

extern crate glutin_window;
extern crate piston;

extern crate graphics;
extern crate opengl_graphics;

use glutin_window::GlutinWindow;
use piston::WindowSettings;

use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::RenderEvent;

use opengl_graphics::{GlGraphics, OpenGL};

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Roguelike", [512; 2]).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut gl = GlGraphics::new(opengl);

    let RED = [1.0, 0.0, 0.0, 1.0];
    let GREEN = [0.0, 1.0, 0.0, 1.0];
    let BLUE = [0.0, 0.0, 1.0, 1.0];
    let WHITE = [1.0; 4];

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |_c, g| {
                graphics::clear(BLUE, g);
            });
        }
    }
}
