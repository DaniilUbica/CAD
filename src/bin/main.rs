extern crate CAD;

use CAD::painter::*;
use CAD::gui::*;

fn main() {
    let app = fltk::app::App::default();
    let mut wnd = init_window();
    fltk::prelude::WidgetExt::show(&mut wnd);
    app.run().unwrap();
    // let coordinates = vec![(400, 400), (100, 400), (100, 200), (200, 200), (0, 100), (0, 0), (400, 0), (400, 200), (300, 300)];
    // let connections = vec![(0, 1), (1, 2), (2, 3), (2, 4), (4, 5), (5, 6), (6, 7), (7, 8)];
    // draw_figure(&coordinates, &connections);
}