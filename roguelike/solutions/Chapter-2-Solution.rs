#![crate_name = "roguelike"]
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

type Colour = [f32; 4];

const RED: Colour = [1.0, 0.0, 0.0, 1.0];
const GREEN: Colour = [0.0, 1.0, 0.0, 1.0];
const BLUE: Colour = [0.0, 0.0, 1.0, 1.0];
const WHITE: Colour = [1.0; 4];
const BLACK: Colour = [0.0, 0.0, 0.0, 1.0];

const WINDOW_SIZE: i32 = 512;
const PIXEL_SIZE: f64 = 32.0;
const WORLD_SIZE: i32 = WINDOW_SIZE / PIXEL_SIZE as i32;

#[derive(Clone)]
struct Tile {
    colour: Colour,
}

impl Tile {
    pub fn empty() -> Self {
        Tile { colour: WHITE }
    }

    pub fn wall() -> Self {
        Tile { colour: BLACK }
    }
}

type Map = Vec<Vec<Tile>>;

fn make_map() -> Map {
    let mut map = vec![vec![Tile::empty(); WORLD_SIZE as usize]; WORLD_SIZE as usize];
    map[WORLD_SIZE as usize / 2][WORLD_SIZE as usize / 2] = Tile::wall();
    map
}

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Roguelike", [512; 2]).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut gl = GlGraphics::new(opengl);
    let map = make_map();
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, g| {
                graphics::clear(BLUE, g);

                for i in 0..WORLD_SIZE {
                    for j in 0..WORLD_SIZE {
                        let pos: [f64; 4] = [
                            PIXEL_SIZE * i as f64,
                            PIXEL_SIZE * j as f64,
                            PIXEL_SIZE * (i + 1) as f64,
                            PIXEL_SIZE * (j + 1) as f64,
                        ];
                        graphics::Rectangle::new(map[i as usize][j as usize].colour).draw(
                            pos,
                            &c.draw_state,
                            c.transform,
                            g,
                        );
                    }
                }
            });
        }
    }
}
