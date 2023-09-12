use std::{
	cmp::{max, min},
	num::NonZeroU32,
	ops::Add,
};

use softbuffer::{Context, Surface};
use winit::{
	dpi::{PhysicalPosition, PhysicalSize},
	event_loop::EventLoop,
	window::{Window, WindowId},
};

#[derive(Clone, Copy)]
pub struct Color(pub [u8; 3]);

impl Color {
	fn to_u32(&self) -> u32 {
		(self.0[0] as u32) << 2 * u8::BITS
			| (self.0[1] as u32) << u8::BITS
			| self.0[2] as u32
	}
}

impl From<u32> for Color {
	fn from(value: u32) -> Self {
		Self([
			(value >> 2 * u8::BITS) as u8,
			(value >> u8::BITS) as u8,
			value as u8,
		])
	}
}

impl Add for Color {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self([
			self.0[0] / 2 + rhs.0[0] / 2,
			self.0[1] / 2 + rhs.0[1] / 2,
			self.0[2] / 2 + rhs.0[2] / 2,
		])
	}
}

pub struct Area(pub (i32, i32), (i32, i32));

pub struct ColorWindow {
	window: Window,
	color: Color,
	#[allow(unused)]
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

	pub fn area(&self) -> Area {
		let PhysicalPosition { x, y } = self.window.inner_position().unwrap();
		let PhysicalSize { width, height } = self.window.inner_size();

		Area(
			(x, y),
			(x + width as i32, y + height as i32),
		)
	}

	pub fn draw(&mut self, areas: &[Area]) {
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

		for &Area((x1, y1), (x2, y2)) in areas {
			for x in x1..x2 {
				for y in y1..y2 {
					let idx = y as usize * width as usize + x as usize;
					if buf.len() > idx {
						buf[idx] =
							(Color::from(buf[idx]) + self.color).to_u32();
					}
				}
			}
		}

		buf.present().unwrap();
	}

	pub fn is_overlapping_with(&self, cw: &Self) -> Option<Area> {
		let a1 = self.area();
		let a2 = cw.area();

		let x1 = max(a1.0 .0, a2.0 .0);
		let x2 = min(a1.1 .0, a2.1 .0);
		let y1 = max(a1.0 .1, a2.0 .1);
		let y2 = max(a1.1 .1, a2.1 .1);

		if x1 < x2 && y1 < y2 {
			Some(Area(
				(x1 - a1.0 .0, y1 - a1.0 .1),
				(x2 - a1.0 .0, y2 - a1.0 .1),
			))
		} else {
			None
		}
	}
}
