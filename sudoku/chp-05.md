# Sudoku tutorial
by Sven Nilsen, 2017

# Chapter 5

Add the following code to `GameboardView::draw`:

```rust
  /// Draw gameboard.
  pub fn draw<G: Graphics>(&self, controller: &GameboardController, c: &Context, g: &mut G) {
      use graphics::{Line, Rectangle};

      let ref settings = self.settings;
      let board_rect = [
          settings.position[0], settings.position[1],
          settings.size, settings.size,
      ];

      // Draw board background.
      Rectangle::new(settings.background_color)
          .draw(board_rect, &c.draw_state, c.transform, g);

      // Draw cell borders.
      let cell_edge = Line::new(settings.cell_edge_color, settings.cell_edge_radius);
      for i in 0..9 {
          // Skip lines that are covered by sections.
          if (i % 3) == 0 {continue;}

          let x = settings.position[0] + i as f64 / 9.0 * settings.size;
          let y = settings.position[1] + i as f64 / 9.0 * settings.size;
          let x2 = settings.position[0] + settings.size;
          let y2 = settings.position[1] + settings.size;

          let vline = [x, settings.position[1], x, y2];
          cell_edge.draw(vline, &c.draw_state, c.transform, g);

          let hline = [settings.position[0], y, x2, y];
          cell_edge.draw(hline, &c.draw_state, c.transform, g);
      }

      // Draw section borders.
      let section_edge = Line::new(settings.section_edge_color, settings.section_edge_radius);
      for i in 0..3 {
          // Set up coordinates.
          let x = settings.position[0] + i as f64 / 3.0 * settings.size;
          let y = settings.position[1] + i as f64 / 3.0 * settings.size;
          let x2 = settings.position[0] + settings.size;
          let y2 = settings.position[1] + settings.size;

          let vline = [x, settings.position[1], x, y2];
          section_edge.draw(vline, &c.draw_state, c.transform, g);

          let hline = [settings.position[0], y, x2, y];
          section_edge.draw(hline, &c.draw_state, c.transform, g);
      }

      // Draw board edge.
      Rectangle::new_border(settings.board_edge_color, settings.board_edge_radius)
          .draw(board_rect, &c.draw_state, c.transform, g);
  }
```

Piston-Graphics splits data into high and low frequency usage.
`Line` and `Rectangle` store color and edge information,
but the coordinates are passed to the `.draw` method call.
This makes it easy to reuse these objects by declaring them before loops.

`Line` stores the coordinates in the format `[x1, y1, x2, y2]`.
`Rectangle` stores the coordinates in the format `[x, y, w, h]`.

In addition to the shape, the `.draw` method requires the draw state
from `Context`, a matrix transform (often `c.transform`) and the graphics backend.

If you are familiar with other APIs like Cairo or GDI+, you might think
the way Piston-Graphics does drawing is somewhat unfamiliar.
In these APIs the context and the object that renders is the same thing.
In Piston-Graphics, `Context` is separated from the object implementing
`Graphics`.

By manipulating the `Context` object, one can use the default stack as
a way to push/pop transforms.

The draw state stores things like scissor rectangle, stencil usage and
blending settings.

The matrix transform is use to translate, rotate, scale etc. the shape.
