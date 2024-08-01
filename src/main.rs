
use minifb::{Key, MouseButton, Window, WindowOptions};
use serde::{Deserialize, Serialize};
use std::fs;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

#[derive(Serialize, Deserialize, Debug)]
struct Canvas {
    pixels: Vec<u8>,
}

impl Canvas {
    fn new() -> Self {
        Self {
            pixels: vec![0; WIDTH * HEIGHT],
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, value: u8) {
        if x < WIDTH && y < HEIGHT {
            self.pixels[y * WIDTH + x] = value;
        }
    }

    fn save_to_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let serialized = serde_json::to_string(&self)?;
        fs::write(filename, serialized)?;
        Ok(())
    }

    fn load_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let data = fs::read_to_string(filename)?;
        let canvas: Canvas = serde_json::from_str(&data)?;
        Ok(canvas)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut canvas = Canvas::new();
    let mut window = Window::new("Canvas - Press ESC to exit", WIDTH, HEIGHT, WindowOptions::default())
        .expect("Unable to open window");

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.get_mouse_down(MouseButton::Left) {
            if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
                let x = x as usize;
                let y = y as usize;
                canvas.set_pixel(x, y, 1);
            }
        }

        for (i, &pixel) in canvas.pixels.iter().enumerate() {
            buffer[i] = if pixel == 1 { 0xFFFFFF } else { 0x000000 };
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }

    canvas.save_to_file("canvas.json")?;
    println!("Canvas saved to canvas.json");

    // To load and display the canvas, run this code in a new session
    // let loaded_canvas = Canvas::load_from_file("canvas.json")?;
    // for (i, &pixel) in loaded_canvas.pixels.iter().enumerate() {
    //     buffer[i] = if pixel == 1 { 0xFFFFFF } else { 0x000000 };
    // }
    // window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

    Ok(())
}
