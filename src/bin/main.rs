extern crate cad;

use cad::gui::*;

fn main() {
    let app = fltk::app::App::default();
    let mut wnd = init_window(WINDOW_WIDTH, WINDOW_HEIGHT, &init_input);
    fltk::prelude::WidgetExt::show(&mut wnd);
    app.run().unwrap();
}