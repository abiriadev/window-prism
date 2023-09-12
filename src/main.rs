use std::collections::HashMap;

use window_prism::{Color, ColorWindow};
use winit::{
	event::{Event, WindowEvent},
	event_loop::EventLoop,
};

fn main() -> anyhow::Result<()> {
	let event_loop = EventLoop::new();

	let mut color_windows = HashMap::new();

	for i in 0..3 {
		let color_window = ColorWindow::new(
			&event_loop,
			Color([255 / 3 * i, 255 / 3 * i, 255 / 3 * i]),
		);
		color_windows.insert(color_window.id(), color_window);
	}

	event_loop.run(move |event, _, ctrl_flow| {
		ctrl_flow.set_wait();

		match event {
			Event::WindowEvent { window_id, event } => match event {
				// WindowEvent::Resized(_) => todo!(),
				// WindowEvent::Moved(_) => todo!(),
				WindowEvent::CloseRequested => {
					// Drops the window, causing it to close.
					color_windows.remove(&window_id);

					// Quit entire process if and only if the closed window was the last window.
					if color_windows.is_empty() {
						ctrl_flow.set_exit();
					}
				},

				_ => (),
			},
			// Event::MainEventsCleared => todo!(),
			Event::RedrawRequested(wid) => {
				let Some(color_window) = color_windows.get(&wid) else { return; };

				let areas = color_windows
					.iter()
					.filter(|(&k, _)| k != wid)
					.filter_map(|(_, w)| color_window.is_overlapping_with(w))
					.collect::<Vec<_>>();

				let Some(color_window) = color_windows.get_mut(&wid) else { return; };

				color_window.draw(&areas);
			},
			Event::LoopDestroyed => (),
			_ => (),
		}
	})
}
