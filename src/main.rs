#![windows_subsystem = "windows"]

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

fn draw(canvas: &mut Canvas<Window>, x: i32, y: i32, dimensions: (u32, u32), col: Color) {
    let x_offset = dimensions.0 / 2;
    let y_offset = dimensions.1 / 2;
    let prev_colour = canvas.draw_color();
    canvas.set_draw_color(col);
    canvas
        .draw_point(Point::new(x + x_offset as i32, y + y_offset as i32))
        .unwrap();
    canvas.set_draw_color(prev_colour);
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    const WIN_DIMENSIONS: (u32, u32) = (600, 600);

    let window = video_subsystem
        .window(
            "Rotating donut of course",
            WIN_DIMENSIONS.0,
            WIN_DIMENSIONS.1,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // canvas allows for changing whats on window
    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    // black screen
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    // game loop init
    let mut event_pump = sdl_context.event_pump()?;

    // variables for torus rotation
    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;
    #[allow(dead_code)]
    const WHITE: Color = Color::RGB(255, 255, 255);
    'running: loop {
        // get inputs here
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // game loop here
        // change colour
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let r1 = 80.0; // radius of inner circle.
        let r2 = 150.0; // radius of torus
        const K2: f64 = 5000.0;
        let k1: f64 = WIN_DIMENSIONS.0 as f64 * K2 * 3.0 / (8.0 * (r1 + r2));
        for i in (0..628).step_by(5) {
            // 628 is pi * 2

            let cos_t = (i as f64 / 100.0).cos();
            let sin_t = (i as f64 / 100.0).sin();
            let x2 = r2 + r1 * cos_t;
            let y2 = r1 * sin_t;

            // draw the other circles
            for j in (0..628).step_by(4) {
                let cos_p = (j as f64 / 100.0).cos();
                let sin_p = (j as f64 / 100.0).sin();
                // abslutely insane rotation matrices lmao
                // https://www.cantorsparadise.com/why-a-spinning-donut-is-a-pure-math-e4dccc6294b0
                // also check out Andy Sloane's explanation
                // when the rotations seem to compress when turning a certain way it means that the below
                // equations are incorrect
                let x = x2 * (b.cos() * cos_p + a.sin() * b.sin() * sin_p) - y2 * a.cos() * b.sin();
                let y = x2 * (cos_p * b.sin() - b.cos() * a.sin() * sin_p) + y2 * a.cos() * b.cos();
                let iz = 1.0 / (K2 + r1 * a.sin() * sin_t + a.cos() * sin_p * x2); // inverse z

                // final point coords
                let xp = (x * k1 * iz).floor() as i32;
                let yp = (-y * k1 * iz).floor() as i32;

                // luminance
                let l = cos_p * cos_t * b.sin() - a.cos() * cos_t * sin_p - a.sin() * sin_t
                    + b.cos() * (a.cos() * sin_t - cos_t * a.sin() * sin_p);

                if l > 0.0 {
                    let rl = -(l * 180.0).round() as i32; // rounded luminance
                    let l_index = (255 - rl) as u8;
                    draw(
                        &mut canvas,
                        xp,
                        yp,
                        WIN_DIMENSIONS,
                        Color::RGB(l_index, l_index, l_index),
                    );
                }
            }
        }
        if a != 2.0 {
            a += 0.01;
            b += 0.005;
        } else {
            a = 0.0;
            b = 0.0;
        }
        canvas.present(); // push to canvas
    }
    Ok(())
}
