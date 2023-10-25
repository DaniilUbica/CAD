use std::borrow::BorrowMut;

use fltk::image::PngImage;
use fltk::{*, image::Image};
use fltk::button::Button;
use fltk::frame::Frame;
use fltk::window::Window;
use fltk::prelude::*;

use crate::{parse_numbers, draw_figure, painter};

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
const OFFSET: i32 = 50;

pub fn init_window() -> window::DoubleWindow {
    let window = Window::new(350, 150, WINDOW_WIDTH, WINDOW_HEIGHT, "SAPR-BAR");

    init_input();

    window.end();
    window
}

fn init_input() {
    let points_amount = input::Input::new(150, 50, 150, 25, "Количество точек: ");
    let points = input::Input::new(150, 100, 150, 25, "Координаты точек: ");

    let mut info = Frame::new(250, 575, 500, 25, "");
    let mut points_vec= vec![];
    let mut confirm_btn = Button::new(325, 150, 150, 25, "Подтвердить");

    let mut figure_frame = Frame::new(OFFSET, 200, WINDOW_WIDTH - OFFSET * 2, WINDOW_HEIGHT / 2, "");

    confirm_btn.set_callback({       // seychas budet pizdec...
        let mut err_msg = "";

        move |_| {
            let p = match points_amount.value().parse::<i32>() {
                Ok(p) => p,
                Err(_) => -1,
            };

            match parse_numbers(points.value(), p as usize, "Ошибка! Вы ввели слишком мало значений") {
                Ok(v) => { *points_vec.borrow_mut() = v; err_msg = "" },
                Err(e) => { err_msg = e.0; points_vec.clear() },
            };

            if p < 0 {
                err_msg = "Ошибка! Вы ввели неверное количество элементов";
            }

            let mut p_copy = vec![];
            for i in &points_vec {
                p_copy.push((i.0 * 100, i.1 * 100));
            }

            if err_msg.is_empty() {
                draw_figure(&points_vec[..]);
                let mut figure = PngImage::load("./out.png").unwrap();
                figure.scale(WINDOW_WIDTH - OFFSET * 2, WINDOW_HEIGHT / 2, true, true);
                figure_frame.set_image(Some(figure));
            }

            info.set_label(err_msg);
            info.set_color(enums::Color::Red);
        }
    });

}