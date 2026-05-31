use glam::Vec2;
use sdl3::{
	rect::Rect,
	render::{Canvas, FPoint, Texture},
	video::Window,
};

pub struct Sprite<'a> {
	pub size: Vec2,
	pub rotation: f32,
	texture: &'a Texture<'a>,
}

impl<'a> Sprite<'a> {
	pub fn from_texture(texture: &'a Texture<'a>) -> Self {
		let width = texture.width();
		let height = texture.height();

		Self {
			size: Vec2::new(width as f32, height as f32),
			rotation: 0_f32,
			texture,
		}
	}

	pub fn draw(&self, canvas: &mut Canvas<Window>, pos: Vec2) {
		let rect = Rect::new(pos.x as i32, pos.y as i32, self.size.x as u32, self.size.y as u32);
		let center = Some(FPoint::new(self.size.x / 2_f32, self.size.y / 2_f32));

		canvas
			.copy_ex(&self.texture, None, rect, self.rotation as f64, center, false, false)
			.unwrap();
	}
}
