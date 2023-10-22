use plotters::prelude::*;

use image::{ImageBuffer, Rgb, RgbImage};

pub fn draw(points: Vec<(u32, u32)>) {
    let width = 800;
    let height = 600;
    let mut image: RgbImage = ImageBuffer::new(width, height);

    let color = Rgb([255, 0, 0]);

    draw_polygon(&mut image, &points, color);

    image.save("output.png").unwrap();
}

fn draw_polygon(image: &mut RgbImage, points: &[(u32, u32)], color: Rgb<u8>) {
    for i in 0..points.len() - 1 {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % points.len()];
        draw_line(image, x1, y1, x2, y2, color);
    }
}

fn draw_line(image: &mut RgbImage, x1: u32, y1: u32, x2: u32, y2: u32, color: Rgb<u8>) {
    let dx = (x2 as i32 - x1 as i32).abs();
    let dy = (y2 as i32 - y1 as i32).abs();
    let sx: i32 = if x1 < x2 { 1 } else { -1 };
    let sy: i32 = if y1 < y2 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = x1 as i32;
    let mut y = y1 as i32;

    while x != x2 as i32 || y != y2 as i32 {
        image.put_pixel(x as u32, y as u32, color);

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}
