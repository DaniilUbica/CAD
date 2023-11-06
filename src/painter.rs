use image::{ImageBuffer, Rgb, RgbImage, imageops};
use rusttype::{Font, Scale};

use crate::contains_in_vec;

pub const OUT_FILE_NAME: &str = "out.png";

const OFFSET: u32 = 50;
const POINT_RADIUS: u32 = 3;

const RED: Rgb<u8> = Rgb([255, 0, 0]);
const GREEN: Rgb<u8> = Rgb([0, 255, 0]);
const BLUE: Rgb<u8> = Rgb([0, 0, 255]);
const BLACK: Rgb<u8> = Rgb([0, 0, 0]);
const WHITE: Rgb<u8> = Rgb([255, 255, 255]);

pub fn draw_figure(rects: &[(usize, usize)], point_loads: &[(i32, i32)], distributed_loads: &[(i32, i32)]) -> (u32, u32) {
    let mut total_width = (OFFSET * 2) as usize;
    for i in rects {
        total_width += i.1;
    }

    let height = rects.iter().max_by_key(|&(value, _)| value).unwrap().0 + OFFSET as usize;

    let mut image_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_pixel(total_width as u32, height as u32, WHITE);

    let mut rects_points = vec![];
    let mut rects_heights = vec![];

    let mut pos_x = OFFSET as usize;
    for i in rects {
        let tmp = pos_x;

        draw_rectangle_outline(&mut image_buffer, pos_x, (height - i.0) / 2, i.1, i.0, BLACK);

        pos_x += i.1;
        rects_points.push((tmp, pos_x));
        rects_heights.push((height) / 2);
    }

    for i in 0..distributed_loads.len() {
        let start_x = rects_points[distributed_loads[i].0 as usize - 1].1;
        let end_x = rects_points[distributed_loads[i].0 as usize - 1].0;
        let h = rects_heights[distributed_loads[i].0 as usize - 1];

        if distributed_loads[i].1 > 0 {
            draw_line(&mut image_buffer, start_x, h,end_x, h, RED);
        }
        else {
            draw_line(&mut image_buffer, start_x, h,end_x, h, BLUE);
        }
    }

    for i in 0..point_loads.len() {
        let x;
        let y = rects_heights[point_loads[i].0 as usize - 1];
        
        if i == point_loads.len() - 1 && point_loads.len() > 1 || i == 0 {
            x = rects_points[point_loads[i].0 as usize - 1].1;
        }
        else {
            x = rects_points[point_loads[i].0 as usize - 1].0;
        }

        if point_loads[i].1 > 0 {
            draw_circle(&mut image_buffer, x as i32, y as i32, POINT_RADIUS as i32, RED);
        }
        else {
            draw_circle(&mut image_buffer, x as i32, y as i32, POINT_RADIUS as i32, BLUE);
        }
    }

    image_buffer.save(OUT_FILE_NAME).unwrap();

    (total_width as u32, height as u32)
}

fn draw_line(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x1: usize, y1: usize, x2: usize, y2: usize, color: Rgb<u8>) {
    let dx = (x2 as i32 - x1 as i32).abs();
    let dy = (y2 as i32 - y1 as i32).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut x = x1 as i32;
    let mut y = y1 as i32;

    if dx > dy {
        let mut err = dx / 2;
        loop {
            img.put_pixel(x as u32, y as u32, color);
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
            img.put_pixel(x as u32, y as u32, color);
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

fn draw_rectangle_outline(image_buffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x: usize, y: usize, width: usize, height: usize, color: Rgb<u8>) {
    for i in x..(x + width) {
        image_buffer.put_pixel(i as u32, y as u32, color);
        image_buffer.put_pixel(i as u32, y as u32 + height as u32 - 1, color);
    }

    for j in y..(y + height) {
        image_buffer.put_pixel(x as u32, j as u32, color);
        image_buffer.put_pixel(x as u32 + width as u32 - 1, j as u32, color);
    }
}

fn draw_text(image: &mut RgbImage, font: &Font<'static>, scale: Scale, x: u32, y: u32, color: Rgb<u8>, text: &str) {
    let glyphs: Vec<_> = font.layout(text, scale, rusttype::point(x as f32, y as f32)).collect();

    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|gx, gy, _v| {
                let x = gx + bounding_box.min.x as u32;
                let y = gy + bounding_box.min.y as u32;
                let pixel = image.get_pixel_mut(x, y);
                *pixel = color;
            });
        }
    }
}

fn draw_circle(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, center_x: i32, center_y: i32, radius: i32, color: Rgb<u8>) {
    for mut x in (center_x - radius)..(center_x + radius) {
        for mut y in (center_y - radius)..(center_y + radius) {
            let dx = x - center_x;
            let dy = y - center_y;
            if dx * dx + dy * dy <= radius * radius {
                if x < 0 {
                    x *= -1;
                }
                if y < 0 {
                    y *= -1;
                }
                image.put_pixel(x as u32, y as u32, color);
            }
        }
    }
}

fn draw_arrow(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, start_x: u32, start_y: u32, end_x: u32, end_y: u32) {
    imageproc::drawing::draw_line_segment_mut(image, (start_x as f32, start_y as f32), (end_x as f32, end_y as f32), RED);

    let angle = ((end_y as f32 - start_y as f32).atan2(end_x as f32 - start_x as f32) + std::f32::consts::PI * 2.0) % (std::f32::consts::PI * 2.0);

    let arrow_length = 10;
    let arrow_angle = std::f32::consts::PI / 6.0;
    let arrow_start_x = end_x as f32 - (arrow_length as f32 * angle.cos());
    let arrow_start_y = end_y as f32 - (arrow_length as f32 * angle.sin());
    let arrow_end_x = end_x as f32 - ((arrow_length as f32 * arrow_angle.cos()) + (arrow_length as f32 * (angle + arrow_angle).cos()));
    let arrow_end_y = end_y as f32 - ((arrow_length as f32 * arrow_angle.sin()) + (arrow_length as f32 * (angle + arrow_angle).sin()));
    imageproc::drawing::draw_line_segment_mut(image, (arrow_start_x, arrow_start_y), (arrow_end_x, arrow_end_y), RED);

    let arrow_end_x = end_x as f32 - ((arrow_length as f32 * arrow_angle.cos()) + (arrow_length as f32 * (angle - arrow_angle).cos()));
    let arrow_end_y = end_y as f32 - ((arrow_length as f32 * arrow_angle.sin()) + (arrow_length as f32 * (angle - arrow_angle).sin()));
    imageproc::drawing::draw_line_segment_mut(image, (arrow_start_x, arrow_start_y), (arrow_end_x, arrow_end_y), RED);
}