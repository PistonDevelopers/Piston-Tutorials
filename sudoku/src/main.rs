#![deny(missing_docs)]

//! A Sudoku game.

use glutin_window::GlutinWindow;
use opengl_graphics::{
    Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings,
};
use piston::event_loop::{EventSettings, Events};
use piston::{EventLoop, RenderEvent, WindowSettings};

pub use crate::gameboard::Gameboard;
pub use crate::gameboard_controller::GameboardController;
pub use crate::gameboard_view::{GameboardView, GameboardViewSettings};

mod gameboard;
mod gameboard_controller;
mod gameboard_view;

static FONT: &str = "/usr/share/fonts/TTF/DejaVuSans.ttf";

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Sudoku", (640, 480))
        .exit_on_esc(true)
        .graphics_api(opengl)
        .vsync(true);
    let mut window: GlutinWindow =
        settings.build().expect("Could not create window");
    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    let args: Vec<_> = std::env::args().collect();
    let infile = args.get(1).expect("usage: sudoku SDM_FILE");

    let gameboard = Gameboard::load_sdm(infile);
    let mut gameboard_controller = GameboardController::new(gameboard);
    let gameboard_view_settings = GameboardViewSettings::new();
    let gameboard_view = GameboardView::new(gameboard_view_settings);

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new(FONT, (), texture_settings)
        .expect(&format!("failed to load font `{}`", FONT));

    while let Some(e) = events.next(&mut window) {
        gameboard_controller.event(
            gameboard_view.settings.position,
            gameboard_view.settings.size,
            &e,
        );
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;

                clear([1.0; 4], g);
                gameboard_view.draw(&gameboard_controller, glyphs, &c, g);
            });
        }
    }
}
