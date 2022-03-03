use std::io;
use std::io::Write;
use std::fs::File;

// Write the pixels to a ppm file
fn write_ppm(file: &mut File, pixels: &[u32], width: usize) -> io::Result<()> {
    let w = width;
    let h = pixels.len()/w; 
    write!(file, "P6\n{} {} 255\n", w, h)?;
    for pixel in pixels {
        let r = ((pixel >> 8*2) & 0xFF) as u8;
        let g = ((pixel >> 8*1) & 0xFF) as u8;
        let b = ((pixel >> 8*0) & 0xFF) as u8;
        file.write(&[r,g,b])?;
    }
    Ok(())
}

fn main() {
    // SCREEN SIZE CONSTANTS
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;

    // Generate the pixels
    // let pixels = [0xFF0000; WIDTH*HEIGHT];
    let mut pixels: [u32; WIDTH*HEIGHT] = [0xFFFFFF; WIDTH*HEIGHT]; 
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let u = x as f32 / WIDTH as f32;
            let v = y as f32 / HEIGHT as f32;
            let _y = (u*255.0) as u32; 
            let _x = (v*255.0) as u32; 
            pixels[y*WIDTH + x] = (_x << 8*2) | (_y << 8*1);
        }
    }

    // Generate a File
    let mut f = File::create("output.ppm").unwrap();
    write_ppm(&mut f,&pixels,WIDTH).expect("app failed"); 
}
