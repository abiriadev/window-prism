use window_prism::{Color, ColorWindow};
use winit::{
	event::{Event, WindowEvent},
	event_loop::EventLoop,
};

fn main() -> anyhow::Result<()> {
	let event_loop = EventLoop::new();

	let mut color_window =
		ColorWindow::new(&event_loop, Color([100, 100, 100]));

	event_loop.run(move |event, _, ctrl_flow| {
		ctrl_flow.set_wait();

		match event {
			Event::WindowEvent { window_id, event }
				if window_id == color_window.id() =>
				match event {
					// WindowEvent::Resized(_) => todo!(),
					// WindowEvent::Moved(_) => todo!(),
					WindowEvent::CloseRequested => ctrl_flow.set_exit(),
					_ => (),
				},
			// Event::MainEventsCleared => todo!(),
			Event::RedrawRequested(_) => {
				color_window.draw();
			},
			Event::LoopDestroyed => (),
			_ => (),
		}
	})
}
