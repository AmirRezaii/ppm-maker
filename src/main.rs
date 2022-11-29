use std::io;
use std::io::Write;
use std::fs::File;

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
            let dx = (_x as i32 - x as i32);
            let dy = (_y as i32 - y as i32);
            if dx*dx + dy*dy < (r*r) as i32 {
                pixels[_y*width + _x] = color;
            }
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
    circle(&mut pixels, WIDTH, WIDTH/2, HEIGHT/2, 70, rgb(255,255,0));

    // Generate a File
    let mut f = File::create("output.ppm").unwrap();
    write_ppm(&mut f,&pixels,WIDTH).expect("writing to file failed");
}
