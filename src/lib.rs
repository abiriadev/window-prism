use std::{num::NonZeroU32, ops::Add};

use softbuffer::{Context, Surface};
use winit::{
	dpi::PhysicalSize,
	event_loop::EventLoop,
	window::{Window, WindowId},
};

pub struct Color(pub [u8; 3]);

impl Color {
	fn to_u32(&self) -> u32 {
		(self.0[0] as u32) << 2 * u8::BITS
			| (self.0[1] as u32) << u8::BITS
			| self.0[2] as u32
	}
}

impl Add for Color {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self([
			(self.0[0] + rhs.0[0]) / 2,
			(self.0[1] + rhs.0[1]) / 2,
			(self.0[2] + rhs.0[2]) / 2,
		])
	}
}

pub struct ColorWindow {
	window: Window,
	color: Color,
	ctx: Context,
	surface: Surface,
}

impl ColorWindow {
	pub fn new(event_loop: &EventLoop<()>, color: Color) -> Self {
		let window = Window::new(&event_loop).unwrap();

		let ctx = unsafe { Context::new(&window) }.unwrap();

		let surface = unsafe { Surface::new(&ctx, &window) }.unwrap();

		Self {
			window,
			color,
			ctx,
			surface,
		}
	}

	pub fn id(&self) -> WindowId { self.window.id() }

	pub fn draw(&mut self) {
		let PhysicalSize { width, height } = self.window.inner_size();
		self.surface
			.resize(
				NonZeroU32::new(width).unwrap(),
				NonZeroU32::new(height).unwrap(),
			)
			.unwrap();

		let mut buf = self.surface.buffer_mut().unwrap();

		let color = self.color.to_u32();

		buf.fill(color);
		buf.present().unwrap();
	}
}
