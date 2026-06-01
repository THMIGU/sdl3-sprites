// #![windows_subsystem = "windows"]

mod assets;
mod fps;
mod sprite;

use glam::Vec2;
use sdl3::{
	event::Event,
	pixels::Color,
	sys::render::{
		SDL_RenderCoordinatesFromWindow, SDL_RendererLogicalPresentation, SDL_SetRenderVSync,
	},
};
use std::time::{Duration, Instant};

use crate::{assets::Assets, fps::FPS, sprite::Sprite};

const TICK_RATE: f64 = 60_f64;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const GAME_WIDTH: u32 = 320;
const GAME_HEIGHT: u32 = 180;

fn main() {
	let sdl_context = sdl3::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem
		.window("sdl3-sprites", WINDOW_WIDTH, WINDOW_HEIGHT)
		.position_centered()
		.resizable()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas();

	unsafe {
		SDL_SetRenderVSync(canvas.raw(), 1);
	}

	canvas
		.set_logical_size(GAME_WIDTH, GAME_HEIGHT, SDL_RendererLogicalPresentation::LETTERBOX)
		.unwrap();

	let texture_creator = canvas.texture_creator();

	let assets = Assets::new(&texture_creator);
	let mut sprite = Sprite::from_texture(assets.get_texture("ball"));

	let mut event_pump = sdl_context
		.event_pump()
		.unwrap();

	let mut last_frame = Instant::now();
	let mut accumulator = Duration::new(0, 0);
	let tick_time = Duration::from_secs_f64(1_f64 / TICK_RATE);

	let mut fps = FPS::new();

	'running: loop {
		let now = Instant::now();
		let frame_duration = now.duration_since(last_frame);
		accumulator += frame_duration;
		last_frame = now;

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {
					..
				} => break 'running,
				_ => {}
			}
		}

		while accumulator >= tick_time {
			let mouse = event_pump.mouse_state();
			let mut mouse_x = 0_f32;
			let mut mouse_y = 0_f32;

			unsafe {
				SDL_RenderCoordinatesFromWindow(
					canvas.raw(),
					mouse.x(),
					mouse.y(),
					&mut mouse_x,
					&mut mouse_y,
				);
			}

			let dx = 160_f32 - mouse_x;
			let dy = 90_f32 - mouse_y;

			let angle = dy.atan2(dx);
			sprite.rotation = angle.to_degrees();

			accumulator -= tick_time;
		}

		let display_fps = fps.fps(frame_duration);

		canvas
			.window_mut()
			.set_title(&format!("sdl3-sprites | {:.0} FPS", display_fps))
			.unwrap();

		canvas.set_draw_color(Color::RED);
		canvas.clear();

		canvas.set_draw_color(Color::WHITE);
		sprite.draw(&mut canvas, Vec2::new(135_f32, 65_f32));

		canvas.present();
	}
}
