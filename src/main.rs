mod algo;
mod app;
mod cert;
mod tabs;

use gpui::{px, size, App, Application, Bounds, WindowBounds, WindowOptions, AppContext};

use crate::app::DevToolsApp;

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1024.), px(768.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_window, cx| cx.new(|cx| DevToolsApp::new(cx)),
        )
        .unwrap();
    });
}
