use std::borrow::BorrowMut;

use fltk::image::PngImage;
use fltk::*;
use fltk::button::Button;
use fltk::frame::Frame;
use fltk::window::Window;
use fltk::prelude::*;

use crate::{parse_numbers, draw_figure, OUT_FILE_NAME, parse_loads, parse_single_numbers, save_rects, save_loads, read_rects, read_loads};

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

    let e = input::Input::new(WINDOW_WIDTH / 2 - INPUT_WIDTH / 2, 150, INPUT_WIDTH, INPUT_HEIGHT, "E: ");
    let k = input::Input::new(WINDOW_WIDTH / 2 - INPUT_WIDTH / 2, 200, INPUT_WIDTH, INPUT_HEIGHT, "k: ");

    let r_loads = input::Input::new(WINDOW_WIDTH / 2 - INPUT_WIDTH / 2, 250, INPUT_WIDTH, INPUT_HEIGHT, "Распределенные нагрузки: ");
    let loads = input::Input::new(WINDOW_WIDTH / 2 - INPUT_WIDTH / 2, 300, INPUT_WIDTH, INPUT_HEIGHT, "Сосредоточенные нагрузки: ");

    let name = input::Input::new(WINDOW_WIDTH / 2 - INPUT_WIDTH / 2, 350, INPUT_WIDTH, INPUT_HEIGHT, "Имя файла: ");

    let mut info = Frame::new(WINDOW_WIDTH / 2 - INFO_WIDTH / 2, 575, INFO_WIDTH, INFO_HEIGHT, "");

    let mut confirm_btn = Button::new(WINDOW_WIDTH / 2 - CONF_BUTTON_WIDTH / 2, 400, CONF_BUTTON_WIDTH, CONF_BUTTON_HEIGHT, "Подтвердить");
    let mut read_from_file_btn = Button::new(WINDOW_WIDTH / 2 - CONF_BUTTON_WIDTH / 2, 450, CONF_BUTTON_WIDTH, CONF_BUTTON_HEIGHT, "Считать из файла");

    let mut points_vec= vec![];
    let mut dist_loads_vec = vec![];
    let mut point_loads_vec = vec![];
    let mut e_vec = vec![];
    let mut k_vec = vec![];

    confirm_btn.set_callback({       // seychas budet pizdec...
        let name = name.clone();
        move |_| {
            
            let mut err_msg;
            points_vec.clear();
            dist_loads_vec.clear();
            point_loads_vec.clear();

            let p = match points_amount.value().parse::<i32>() {
                Ok(p) => p,
                Err(_) => -1,
            };

            match parse_numbers(points.value(), p as usize, "Ошибка! Вы ввели слишком мало значений") {
                Ok(v) => { *points_vec.borrow_mut() = v; err_msg = "" },
                Err(e) => { err_msg = e.0; points_vec.clear() },
            };

            match parse_loads(r_loads.value(), "Ошибка в воде распределенных нагрузок") {
                Ok(v) => { *dist_loads_vec.borrow_mut() = v; err_msg = "" },
                Err(e) => { err_msg = e.0; dist_loads_vec.clear() },
            };

            match parse_loads(loads.value(), "Ошибка в воде сосредоточенных нагрузок") {
                Ok(v) => { *point_loads_vec.borrow_mut() = v; err_msg = "" },
                Err(e) => { err_msg = e.0; point_loads_vec.clear() },
            };
            
            match parse_single_numbers(e.value(), p as usize, "Ошибка! Вы ввели слишком мало значений в 'E'") {
                Ok(v) => { *e_vec.borrow_mut() = v; err_msg = "" },
                Err(e) => { err_msg = e.0; e_vec.clear() },
            };

            match parse_single_numbers(k.value(), p as usize, "Ошибка! Вы ввели слишком мало значений в 'k'") {
                Ok(v) => { *k_vec.borrow_mut() = v; err_msg = "" },
                Err(e) => { err_msg = e.0; k_vec.clear() },
            };

            if p < 0 {
                err_msg = "Ошибка! Вы ввели неверное количество элементов";
            }

            let mut p_copy = vec![];
            for i in &points_vec {
                p_copy.push((i.0 * 100, i.1 * 100));
            }

            if err_msg.is_empty() {
                let file_name = name.value();
                save_rects(&points_vec[..], &e_vec[..], &k_vec[..], &file_name);
                save_loads(&point_loads_vec[..], &dist_loads_vec[..], &file_name);

                let size = draw_figure(&points_vec[..], &point_loads_vec[..], &dist_loads_vec[..]);
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
        }
    });

    read_from_file_btn.set_callback({
        move |_| {
            let file_name = name.value();

            let (rects, _, _) = read_rects(&file_name);
            let (point, dist) = read_loads(&file_name);

            let size = draw_figure(&rects[..], &point[..], &dist[..]);
                let mut w = size.0 as i32;
                let mut h = size.1 as i32;
                if w < WINDOW_WIDTH || h < WINDOW_HEIGHT {
                    w = WINDOW_WIDTH;
                    h = WINDOW_HEIGHT
                }
                let mut wnd = init_window(w, h, &init_frame);
                wnd.show();

        }
    });
}

fn init_frame() {
    let mut figure_frame = Frame::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT, "");
    let mut figure = PngImage::load(format!("./{}", OUT_FILE_NAME)).unwrap();
    figure.scale(WINDOW_WIDTH, WINDOW_HEIGHT, true, true);
    figure_frame.set_image(Some(figure));
}