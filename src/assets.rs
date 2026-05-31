use std::collections::HashMap;

use sdl3::{
	image::LoadTexture,
	render::{Texture, TextureCreator},
	video::WindowContext,
};

pub struct Assets<'a> {
	textures: HashMap<String, Texture<'a>>,
}

impl<'a> Assets<'a> {
	pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
		let mut textures: HashMap<String, Texture<'a>> = HashMap::new();

		for entry in std::fs::read_dir("assets").unwrap() {
			let entry = entry.unwrap();
			let path = entry.path();

			if path
				.extension()
				.and_then(|s| s.to_str())
				!= Some("png")
			{
				continue;
			}

			let name = path
				.file_stem()
				.unwrap()
				.to_string_lossy()
				.into_owned();

			let texture = texture_creator
				.load_texture(&path)
				.unwrap();
			textures.insert(name, texture);
		}

		Self {
			textures: textures,
		}
	}

	pub fn get_texture(&self, name: &str) -> &Texture<'a> {
		self.textures
			.get(name)
			.unwrap()
	}
}
