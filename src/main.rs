use nalgebra_glm as glm;
use std::fs::File;
use std::io::{BufWriter, Write};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    let mut imgbuf = vec![vec!([0, 0, 0]; WIDTH as usize); HEIGHT as usize];

    // Definir los puntos del polígono 1
    let points1 = [
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383)
    ];

    // Dibujar el polígono 1
    draw_polygon(&mut imgbuf, &points1, [0, 255, 255], [255, 255, 255]); // Amarillo con borde blanco

    // Guardar la imagen como BMP
    save_as_bmp("poligon1out.bmp", &imgbuf).unwrap();
}

fn draw_polygon(imgbuf: &mut Vec<Vec<[u8; 3]>>, points: &[(i32, i32)], fill_color: [u8; 3], border_color: [u8; 3]) {
    // Algoritmo de escaneo de líneas para rellenar el polígono
    let mut sorted_points = points.to_vec();
    sorted_points.sort_by(|a, b| a.1.cmp(&b.1));

    let min_y = sorted_points[0].1;
    let max_y = sorted_points[sorted_points.len() - 1].1;

    for y in min_y..=max_y {
        let mut nodes = vec![];

        for i in 0..points.len() {
            let j = (i + 1) % points.len();
            let (x0, y0) = points[i];
            let (x1, y1) = points[j];

            if y0 < y1 {
                if y0 <= y && y < y1 {
                    let x = x0 + (y - y0) * (x1 - x0) / (y1 - y0);
                    nodes.push(x);
                }
            } else if y0 > y1 {
                if y1 <= y && y < y0 {
                    let x = x1 + (y - y1) * (x0 - x1) / (y0 - y1);
                    nodes.push(x);
                }
            }
        }

        nodes.sort();

        for i in (0..nodes.len()).step_by(2) {
            if i + 1 < nodes.len() {
                let x0 = nodes[i];
                let x1 = nodes[i + 1];

                for x in x0..x1 {
                    imgbuf[y as usize][x as usize] = fill_color;
                }
            }
        }
    }

    // Dibujar el borde del polígono
    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        draw_line(imgbuf, points[i], points[j], border_color);
    }
}

fn draw_line(imgbuf: &mut Vec<Vec<[u8; 3]>>, start: (i32, i32), end: (i32, i32), color: [u8; 3]) {
    // Algoritmo de Bresenham para dibujar líneas
    let (mut x0, mut y0) = start;
    let (x1, y1) = end;
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x0 >= 0 && x0 < WIDTH as i32 && y0 >= 0 && y0 < HEIGHT as i32 {
            imgbuf[y0 as usize][x0 as usize] = color;
        }
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

fn save_as_bmp(filename: &str, imgbuf: &Vec<Vec<[u8; 3]>>) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    let filesize = 54 + 3 * WIDTH * HEIGHT;
    let mut bmp_header = vec![
        0x42, 0x4D, // Magic number 'BM'
        0, 0, 0, 0, // File size
        0, 0, 0, 0, // Unused
        54, 0, 0, 0, // Offset to pixel array
        40, 0, 0, 0, // DIB header size
        0, 0, 0, 0, // Width
        0, 0, 0, 0, // Height
        1, 0, 24, 0, // Color planes and bits per pixel
        0, 0, 0, 0, // Compression (no compression)
        0, 0, 0, 0, // Image size (no compression)
        0, 0, 0, 0, // Horizontal resolution
        0, 0, 0, 0, // Vertical resolution
        0, 0, 0, 0, // Colors in color table
        0, 0, 0, 0, // Important color count
    ];

    bmp_header[2..6].copy_from_slice(&(filesize as u32).to_le_bytes());
    bmp_header[18..22].copy_from_slice(&(WIDTH as u32).to_le_bytes());
    bmp_header[22..26].copy_from_slice(&(HEIGHT as u32).to_le_bytes());

    writer.write_all(&bmp_header)?;

    for row in imgbuf.iter().rev() {
        for pixel in row {
            writer.write_all(&pixel[..])?;
        }
    }

    Ok(())
}
