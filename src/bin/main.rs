use image::{ImageBuffer, Rgb};

fn draw_figure(coordinates: &[(u32, u32)], connections: &[(usize, usize)]) {
    // Находим максимальные значения координат для определения размеров изображения
    let max_x = coordinates.iter().map(|&(x, _)| x).max().unwrap();
    let max_y = coordinates.iter().map(|&(_, y)| y).max().unwrap();

    // Создаем новое изображение с размерами, основанными на максимальных значениях координат
    let mut img = ImageBuffer::new(max_x + 1, max_y + 1);

    // Рисуем фигуру путем установки цвета пикселей по заданным координатам
    for &(x, y) in coordinates {
        img.put_pixel(x, y, Rgb([0, 0, 0])); // Черный цвет
    }

    // Соединяем точки ребрами
    for &(point1, point2) in connections {
        let (x1, y1) = coordinates[point1];
        let (x2, y2) = coordinates[point2];
        draw_line(&mut img, x1, y1, x2, y2);
    }

    // Нумеруем точки и ребра
    // for (i, &(x, y)) in coordinates.iter().enumerate() {
    //     img.put_pixel(x, y, Rgb([255, 0, 0])); // Красный цвет
    //     let text = format!("P{}", i);
    //     //let (text_width, text_height) = draw_text(&mut img, &text, x, y);
    //     let line_y = y + text_height as u32;
    //     draw_line(&mut img, x, line_y, x, y);
    //     //draw_text(&mut img, &text, x - text_width as u32 / 2, line_y);
    // }

    // Сохраняем изображение в формате PNG
    img.save("figure.png").unwrap();
}

fn draw_line(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x1: u32, y1: u32, x2: u32, y2: u32) {
    let dx = (x2 as i32 - x1 as i32).abs();
    let dy = (y2 as i32 - y1 as i32).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut x = x1 as i32;
    let mut y = y1 as i32;

    if dx > dy {
        let mut err = dx / 2;
        loop {
            img.put_pixel(x as u32, y as u32, Rgb([0, 0, 255])); // Синий цвет
            if x == x2 as i32 {
                break;
            }
            err -= dy;
            if err < 0 {
                y += sy;
                err += dx;
            }
            x += sx;
        }
    } else {
        let mut err = dy / 2;
        loop {
            img.put_pixel(x as u32, y as u32, Rgb([0, 0, 255])); // Синий цвет
            if y == y2 as i32 {
                break;
            }
            err -= dx;
            if err < 0 {
                x += sx;
                err += dy;
            }
            y += sy;
        }
    }
}

// fn draw_text(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, text: &str, x: u32, y: u32) -> (usize, usize) {
//     let font = image::FontCollection::new()
//         .unwrap()
//         .into_font()
//         .unwrap();
//     let text_width = font.width(text);
//     let text_height = font.height();
//     let text_image = ImageBuffer::from_fn(text_width, text_height, |_, _| {
//         Rgb([255, 255, 255]) // Белый цвет
//     });
//     text_image.copy_within((0, 0, text_width, text_height), (0, 0), img, (x as i32 - text_width as i32 / 2, y as i32));
//     image::imageops::overlay(img, &text_image, x as i32 - text_width as i32 / 2, y as i32);
//     (text_width, text_height)
// }

fn main() {
    let coordinates = vec![(400, 400), (100, 400), (100, 200), (200, 200), (0, 100), (0, 0), (400, 0), (400, 200), (300, 300)]; // Пример координат фигуры
    let connections = vec![(0, 1), (1, 2), (2, 3), (2, 4), (4, 5), (5, 6), (6, 7), (7, 8)]; // Пример соединений точек
    draw_figure(&coordinates, &connections);
}