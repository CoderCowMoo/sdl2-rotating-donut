use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

fn draw(canvas: &mut Canvas<Window>, x: i32, y: i32, dimensions: (u32, u32)) -> Result<(), String> {
    let x_offset = dimensions.0 / 2;
    let y_offset = dimensions.1 / 2;
    canvas.draw_point(Point::new(x + x_offset as i32, y + y_offset as i32))
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
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let r1 = 40.0; // radius of inner circle.
        let r2 = 100.0; // radius of torus
        for i in (0..628).step_by(15) {
            // 628 is pi * 2

            let cos_t = (i as f64 / 100.0).cos();
            let sin_t = (i as f64 / 100.0).sin();
            let x2 = r2 + r1 * cos_t;
            let y2 = r1 * sin_t;

            // draw the other circles
            for j in (0..628).step_by(10) {
                let cos_p = (j as f64 / 100.0).cos();
                let sin_p = (j as f64 / 100.0).sin();
                let x = x2 * cos_p;
                let y = y2;
                draw(&mut canvas, x as i32, y as i32, WIN_DIMENSIONS)?;
            }
        }
        canvas.present(); // push to canvas
    }
    Ok(())
}
