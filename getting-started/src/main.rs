extern crate piston;

#[allow(missing_copy_implementations)]
pub struct App {
    rotation: f64,      // Rotation for the square.
}

impl App {
    fn render(&mut self, args: &piston::RenderArgs) {
        use piston::graphics::{ Rectangle, RelativeTransform };

        piston::render_2d_opengl(
            // Calling `render_2d_opengl` inside the closure is unsafe
            unsafe { piston::DANGER::new() },
            Some([0.0, 1.0, 0.0, 1.0]),
            |c, g| {

        // Draw a box rotating around the middle of the screen.
        let center_context = &c.trans((args.width / 2) as f64, (args.height / 2) as f64)
            .rot_rad(self.rotation)
            .trans(-25.0, -25.0);
        Rectangle::new([1.0, 0.0, 0.0, 1.0]).draw([0.0, 0.0, 50.0, 50.0], center_context, g);

            }
        );
    }

    fn update(&mut self, args: &piston::UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    piston::start(
        piston::shader_version::OpenGL::_3_2,
        piston::WindowSettings::default(),
        || {
   
    let mut app = App { rotation: 0.0 }; 
    for e in piston::events() {
        use piston::event::{ RenderEvent, UpdateEvent };

        e.render(|r| app.render(r));
        e.update(|u| app.update(u));
    }

        }
    );
}
