use std::num::NonZeroU32;

use softbuffer::{Context, Surface};
use winit::{
	dpi::PhysicalSize,
	event::{Event, WindowEvent},
	event_loop::EventLoop,
	window::Window,
};

fn main() -> anyhow::Result<()> {
	//
	let event_loop = EventLoop::new();

	let window = Window::new(&event_loop)?;

	let ctx = unsafe { Context::new(&window) }.unwrap();

	let mut surface = unsafe { Surface::new(&ctx, &window) }.unwrap();

	event_loop.run(move |event, _, ctrl_flow| {
		ctrl_flow.set_wait();

		match event {
			Event::WindowEvent { window_id, event }
				if window_id == window.id() =>
				match event {
					// WindowEvent::Resized(_) => todo!(),
					// WindowEvent::Moved(_) => todo!(),
					WindowEvent::CloseRequested => ctrl_flow.set_exit(),
					_ => (),
				},
			// Event::MainEventsCleared => todo!(),
			Event::RedrawRequested(_) => {
				let PhysicalSize { width, height } = window.inner_size();
				surface
					.resize(
						NonZeroU32::new(width).unwrap(),
						NonZeroU32::new(height).unwrap(),
					)
					.unwrap();

				let mut buf = surface.buffer_mut().unwrap();

				buf.fill(0);
				buf.present().unwrap();
			},
			Event::LoopDestroyed => (),
			_ => (),
		}
	})
}
