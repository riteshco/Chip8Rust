use std::env;
use chip8core::*;
use sdl2::event::Event;
use std::fs::File;
use std::io::Read;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;
const TICKS_PER_FRAME: u32 = 10;

fn draw_screen(emu: &Emu , canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0,0,0));
    canvas.clear();

    let screen_buf = emu.get_display_buffer();
    canvas.set_draw_color(Color::RGB(255,255,255));
    for (i , pixel) in screen_buf.iter().enumerate() {
        if *pixel {
            //converting 1D array to 2D coords
            let x = (i % SCREEN_WIDTH) as u32;
            let y = (i / SCREEN_WIDTH) as u32;

            let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32 , SCALE , SCALE);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run path/to/game");
        return;
    }


    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Chip 8 Emulator" , WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut chip8 = Emu::new();
    let mut rom = File::open(&args[1]).expect("Unable to open rom");
    let mut buffer = Vec::new();

    rom.read_to_end(&mut buffer).unwrap();
    chip8.load(&buffer);


    'gameloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'gameloop,
                _ => ()
            }
        }
        for _ in 0..TICKS_PER_FRAME {
            chip8.tick();
        }

        chip8.tick_timers();
        draw_screen(&chip8 , &mut canvas);
    }
}
