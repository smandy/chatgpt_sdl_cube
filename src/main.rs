extern crate sdl2;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;


fn multiply_matrix_vector(matrix: &[[f64; 3]], vector: &[f64; 2]) -> [f64; 2] {
    [
        matrix[0][0] * vector[0] + matrix[0][1] * vector[1] + matrix[0][2],
        matrix[1][0] * vector[0] + matrix[1][1] * vector[1] + matrix[1][2],
    ]
}

pub fn draw_cube(canvas: &mut Canvas<Window>, angle_x: f64, angle_y: f64) -> Result<(), String> {
    // Clear canvas
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    // Cube vertices
    let vertices = [
        [-50.0, -50.0, -50.0],
        [-50.0, -50.0, 50.0],
        [-50.0, 50.0, -50.0],
        [-50.0, 50.0, 50.0],
        [50.0, -50.0, -50.0],
        [50.0, -50.0, 50.0],
        [50.0, 50.0, -50.0],
        [50.0, 50.0, 50.0],
    ];

    // Rotate cube vertices
    let sin_x = angle_x.sin();
    let cos_x = angle_x.cos();
    let sin_y = angle_y.sin();
    let cos_y = angle_y.cos();
    let rotated_vertices: Vec<[f64; 3]> = vertices
        .iter()
        .map(|&v| {
            let x = v[0];
            let y = v[1];
            let z = v[2];

            // Rotate around x-axis
            let new_y = y * cos_x - z * sin_x;
            let new_z = y * sin_x + z * cos_x;

            // Rotate around y-axis
            let new_x = x * cos_y + new_z * sin_y;
            let new_z = -x * sin_y + new_z * cos_y;

            [new_x, new_y, new_z]
        })
        .collect();

    // Draw cube edges
    for i in 0..8 {
        for j in 0..8 {
            if i < j {
                let v1 = rotated_vertices[i];
                let v2 = rotated_vertices[j];

                let ov1 = vertices[i];
                let ov2 = vertices[j];

                let xs = (ov1[0]==ov2[0]) as u8;
                let ys = (ov1[1] == ov2[1]) as u8;
                let zs = (ov1[2] == ov2[2]) as u8;

                if xs + ys + zs == 2 {
                    canvas.thick_line(
                        v1[0] as i16 + SCREEN_WIDTH as i16 / 2,
                        v1[1] as i16 + SCREEN_HEIGHT as i16 / 2,
                        v2[0] as i16 + SCREEN_WIDTH as i16 / 2,
                        v2[1] as i16 + SCREEN_HEIGHT as i16 / 2,
                        1,
                        Color::RGB(0, 255, 255),
                    )?;
                }
            }
        }
    }

    // Update canvas
    canvas.present();

    Ok(())
}



fn main() -> Result<(), String> {
    // Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("3D Cube", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    // Main loop
    let mut angle_x = 0.0;
    let mut angle_y = 0.0;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        // Update angles
        angle_x += 0.01;
        angle_y += 0.03;

        // Draw cube
        draw_cube(&mut canvas, angle_x, angle_y)?;

        // Delay to control frame rate
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}


