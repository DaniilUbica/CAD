use std::borrow::BorrowMut;

use fltk::image::PngImage;
use fltk::{*, image::Image};
use fltk::button::Button;
use fltk::frame::Frame;
use fltk::window::Window;
use fltk::prelude::*;

use crate::{parse_numbers, draw_figure, painter, OUT_FILE_NAME};

pub const WINDOW_WIDTH: i32 = 800;
pub const WINDOW_HEIGHT: i32 = 600;

const INPUT_WIDTH: i32 = 250;
const INPUT_HEIGHT: i32 = 25;

const CONF_BUTTON_WIDTH: i32 = 150;
const CONF_BUTTON_HEIGHT: i32 = 25;

const INFO_WIDTH: i32 = 500;
const INFO_HEIGHT: i32 = 25;

const OFFSET: i32 = 50;

pub fn init_window(width: i32, height: i32, content: &dyn Fn()) -> window::DoubleWindow {
    let window = Window::new(350, 150, width, height, "CAD System");

    content();

    window.end();
    window
}

pub fn init_input() {
    let points_amount = input::Input::new(WINDOW_WIDTH / 2 - INPUT_WIDTH / 2, 50, INPUT_WIDTH, INPUT_HEIGHT, "Количество частей: ");
    let points = input::Input::new(WINDOW_WIDTH / 2 - INPUT_WIDTH / 2, 100, INPUT_WIDTH, INPUT_HEIGHT, "Размеры частей: ");

    let r_loads = input::Input::new(WINDOW_WIDTH / 2 - INPUT_WIDTH / 2, 150, INPUT_WIDTH, INPUT_HEIGHT, "Распределенные нагрузки: ");
    let loads = input::Input::new(WINDOW_WIDTH / 2 - INPUT_WIDTH / 2, 200, INPUT_WIDTH, INPUT_HEIGHT, "Сосредоточенные нагрузки: ");

    let mut info = Frame::new(WINDOW_WIDTH / 2 - INFO_WIDTH / 2, 575, INFO_WIDTH, INFO_HEIGHT, "");

    let mut confirm_btn = Button::new(WINDOW_WIDTH / 2 - CONF_BUTTON_WIDTH / 2, 350, CONF_BUTTON_WIDTH, CONF_BUTTON_HEIGHT, "Подтвердить");
    
    let mut points_vec= vec![];
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
                let size = draw_figure(&points_vec[..]);
                let mut w = size.0 as i32;
                let mut h = size.1 as i32;
                if w < WINDOW_WIDTH || h < WINDOW_HEIGHT {
                    w = WINDOW_WIDTH;
                    h = WINDOW_HEIGHT
                }
                let mut wnd = init_window(w, h, &init_frame);
                wnd.show();
            }

            info.set_label(err_msg);
            info.set_color(enums::Color::Red);
        }
    });
}

fn init_frame() {
    let mut figure_frame = Frame::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT, "");
    let mut figure = PngImage::load(format!("./{}", OUT_FILE_NAME)).unwrap();
    figure.scale(WINDOW_WIDTH, WINDOW_HEIGHT, true, true);
    figure_frame.set_image(Some(figure));
}