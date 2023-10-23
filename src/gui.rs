use fltk::*;
use fltk::app::frame_color;
use fltk::button::Button;
use fltk::frame::Frame;
use fltk::window::Window;
use fltk::prelude::*;

use crate::{parse_points, parse_edges};

pub fn init_window() -> window::DoubleWindow{
    let window = Window::new(350, 150, 800, 600, "SAPR-BAR");

    let points_amount = input::Input::new(150, 50, 150, 25, "Количество точек: ");
    let edges_amount = input::Input::new(500, 50, 150, 25, "Количество рёбер: ");
    let points = input::Input::new(150, 100, 150, 25, "Координаты точек: ");
    let edges = input::Input::new(500, 100, 150, 25, "Связи точек(ребра): ");

    let mut info = Frame::new(400 - 150, 575, 300, 25, "");

    let mut points_vec= vec![];
    let mut edges_vec = vec![];
    
    let mut confirm_btn = Button::new(325, 150, 150, 25, "Подтвердить");

    confirm_btn.set_callback({
        let mut err_msg = "";

        move |_| {
            let p = match points_amount.value().parse::<i32>() {
                Ok(p) => p,
                Err(_) => -1,
            };

            let e = match edges_amount.value().parse::<i32>() {
                Ok(p) => p,
                Err(_) => -1,
            };

            match parse_points(points.value(), p as usize) {
                Ok(v) => points_vec = v,
                Err(e) => { err_msg = e.0; points_vec.clear() },
            };

            match parse_edges(edges.value(), e as usize) {
                Ok(v) => edges_vec = v,
                Err(e) => { err_msg = e.0; edges_vec.clear() },
            };
            
            //info.set_color(Color::Light1);
            info.set_label(err_msg);

            println!("{} {} {:?} {:?} {}", p, e, points_vec, edges_vec, err_msg);
        }
    });


    window.end();
    window
}