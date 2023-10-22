use image::{ImageBuffer, Rgb, RgbaImage, Rgba, RgbImage, imageops};
use rusttype::{Font, Scale};

const OFFSET: u32 = 50;
const POINT_RADIUS: u32 = 5;

pub fn draw_figure(coordinates: &[(u32, u32)], connections: &[(usize, usize)]) {
    let max_x = coordinates.iter().map(|&(x, _)| x).max().unwrap() + 100;
    let max_y = coordinates.iter().map(|&(_, y)| y).max().unwrap() + 100;

    let mut img = ImageBuffer::new(max_x + 50, max_y + 50);
    
    for &(point1, point2) in connections {
        let (x1, y1) = coordinates[point1];
        let (x2, y2) = coordinates[point2];
        draw_circle(&mut img, x1 as i32, y1 as i32, POINT_RADIUS as i32, Rgb([0, 0, 255]));
        draw_circle(&mut img, x2 as i32, y2 as i32, POINT_RADIUS as i32, Rgb([0, 0, 255]));
        draw_line(&mut img, x1, y1, x2, y2);
    }
    
    img = imageops::flip_vertical(&img);

    let font_data = include_bytes!("../arial.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

    let scale = Scale::uniform(24.0);

    for &(x, y) in coordinates {
        // draw text here...
    }

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
            img.put_pixel(x as u32 + OFFSET, y as u32 + OFFSET, Rgb([0, 0, 255]));
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
            img.put_pixel(x as u32 + OFFSET, y as u32 + OFFSET, Rgb([0, 0, 255]));
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

fn draw_text(image: &mut RgbImage, font: &Font<'static>, scale: Scale, x: u32, y: u32, color: Rgb<u8>, text: &str) {
    let glyphs: Vec<_> = font.layout(text, scale, rusttype::point(x as f32, y as f32)).collect();

    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|gx, gy, v| {
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
                image.put_pixel(x as u32 + OFFSET, y as u32 + OFFSET, color);
            }
        }
    }
}