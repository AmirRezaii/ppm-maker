use std::io;
use std::fs;
use std::io::Write;
use std::fs::File;
use std::process::Command;

// Write the pixels to a ppm file
fn write_ppm(file: &mut File, pixels: &[u32], width: usize) -> io::Result<()> {
    let w = width;
    let h = pixels.len()/w; 
    write!(file, "P6\n{} {} 255\n", w, h)?;
    for pixel in pixels {
        let p = hexa(*pixel);
        file.write(&p)?;
    }
    Ok(())
}

fn uvgradient(pixels: &mut [u32], width: usize) {
    let height = pixels.len() / width;
    for y in 0..height {
        for x in 0..width {
            let u = x as f32 / width as f32;
            let v = y as f32 / height as f32;
            let _y = (u*255.0) as u8;
            let _x = (v*255.0) as u8;
            pixels[y*width + x] = rgb(_x, _y, 255);
        }
    }
}

fn rect(pixels: &mut [u32], width: usize, x: usize, y: usize, w: usize, h: usize, color: u32) {
    for _y in y..y+h {
        for _x in x..x+w {
            pixels[_y*width + _x] = color;
        }
    }
}

fn circle(pixels: &mut [u32], width: usize, x: usize, y: usize, r: usize, color: u32) {
    for _y in y-r..y+r {
        for _x in x-r..x+r {
            let dx = _x as i32 - x as i32;
            let dy = _y as i32 - y as i32;
            if dx*dx + dy*dy < (r*r) as i32 {
                pixels[_y*width + _x] = color;
            }
        }
    }
}

fn line(pixels: &mut [u32], width: usize, x1: usize, y1: usize, x2: usize, y2: usize, color: u32) {
    if x2 - x1 != 0 {
        let m = (y2 as f32 - y1 as f32) / (x2 as f32 - x1 as f32);
        for _x in x1..x2 {
            let _y = m * _x as f32 - m * x1 as f32 + y1 as f32;
            pixels[_y as usize * width + _x] = color;
        }
    }
}

fn checker(pixels: &mut [u32], width: usize, cols: usize, rows: usize) {
    let height = pixels.len() / width;
    let w = width/cols;
    let h = height/rows;

    let color1 = 0xFFFFFF;
    let color2 = 0x202020;

    for y in 0..rows {
        for x in 0..cols {
            if (x + y) % 2 == 0 {
                rect(pixels, width, x*w, y*h, w, h, color1);
            } else {
                rect(pixels, width, x*w, y*h, w, h, color2);
            }
        }
    }
}

fn dots(pixels: &mut [u32], width: usize, cols: usize, rows: usize) {
    let height = pixels.len() / width;
    let w = width/cols;
    let h = height/rows;
    let r = 30;

    let color = 0x202020;

    for y in 0..rows {
        for x in 0..cols {
            circle(pixels, width, x*w+r, y*h+r, r, color);
        }
    }
}

fn background(pixels: &mut [u32], color: u32) {
    pixels.fill(color);
}

fn rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 8*2) | ((g as u32) << 8*1) | ((b as u32) << 8*0)
}

fn hexa(h: u32) -> [u8; 3] {
    let r = ((h >> 8*2) & 0xFF) as u8;
    let g = ((h >> 8*1) & 0xFF) as u8;
    let b = ((h >> 8*0) & 0xFF) as u8;
    [r, g, b]
}

fn main() {
    // SCREEN SIZE CONSTANTS
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;

    // Generate the pixels
    let mut pixels: [u32; WIDTH*HEIGHT] = [0; WIDTH*HEIGHT];

    background(&mut pixels, 0xFFFFFF);

    uvgradient(&mut pixels, WIDTH);
    line(&mut pixels, WIDTH, 0, 0, WIDTH, HEIGHT, 0x000000);
    line(&mut pixels, WIDTH, 0, HEIGHT-1, WIDTH-1, 0, 0x000000);
    line(&mut pixels, WIDTH, 0, 0, 100, HEIGHT, 0x000000);
    line(&mut pixels, WIDTH, 100, 0, 200, HEIGHT, 0x000000);
    line(&mut pixels, WIDTH, 200, 0, 300, HEIGHT, 0x000000);

    // Generate a File
    let mut f = File::create("output.ppm").unwrap();
    write_ppm(&mut f,&pixels,WIDTH).expect("writing to file failed");

    let _output = Command::new("convert").args(["output.ppm", "output.png"]).output();
    fs::remove_file("output.ppm").expect("failed to remove ppm file");
}
