use crate::event_loop::EventLoopMessage;
use crate::popup_content::PopupContent;
use crate::utils::get_cursor_pos;
use anyhow::{Context, Result};
use cocoa::{
    appkit::{NSView, NSWindow},
    base::{id, nil, YES},
};
use log::trace;
use tao::{
    dpi::{PhysicalPosition, PhysicalSize},
    event_loop::EventLoop,
    monitor::MonitorHandle,
    platform::macos::{WindowBuilderExtMacOS, WindowExtMacOS},
    window::{Window, WindowBuilder},
};

const MUTED_TEXT: &str = "Muted";
const UNMUTED_TEXT: &str = "Unmuted";

pub fn get_mute_title_text(muted: bool) -> &'static str {
    if muted {
        MUTED_TEXT
    } else {
        UNMUTED_TEXT
    }
}

pub struct Popup {
    window: Window,
    content: PopupContent,
    pub cursor_on_separate_monitor: bool,
}

impl Popup {
    pub fn new(event_loop: &EventLoopMessage, muted: bool) -> Result<Self> {
        let window = WindowBuilder::new()
            .with_title(get_mute_title_text(muted))
            .with_titlebar_hidden(true)
            .with_movable_by_window_background(true)
            .with_always_on_top(true)
            .with_closable(false)
            // .with_content_protection(true)
            .with_decorations(false)
            .with_maximized(false)
            .with_minimizable(false)
            .with_resizable(false)
            .with_inner_size(Popup::get_size())
            .with_visible_on_all_workspaces(true)
            .with_visible(false)
            .with_has_shadow(true)
            .build(event_loop)
            .context("Failed to build window")?;
        window.set_ignore_cursor_events(true)?;

        let content = PopupContent::new("");
        unsafe {
            let ns_view = window.ns_view() as id;
            ns_view.addSubview_(content.textfield);
        };

        let popup = Self {
            window,
            content,
            cursor_on_separate_monitor: false,
        };
        Ok(popup)
    }

    /// TODO: add blur?
    /// https://github.com/rust-windowing/winit/issues/538
    /// https://github.com/servo/core-foundation-rs/blob/master/cocoa/examples/nsvisualeffectview_blur.rs

    pub fn update(&mut self, muted: bool) -> Result<&mut Self> {
        self.window.set_title(get_mute_title_text(muted));
        self.update_placement()?;
        self.update_content(muted)?;
        if muted {
            self.window.set_visible(true);
        }
        Ok(self)
    }

    fn update_content(&mut self, muted: bool) -> Result<&mut Self> {
        let text = get_mute_title_text(muted);
        self.content.set_text(text);
        Ok(self)
    }

    pub fn hide(&mut self) -> Result<&mut Self> {
        self.window.set_visible(false);
        Ok(self)
    }

    pub fn get_size() -> PhysicalSize<i32> {
        PhysicalSize::new(200, 40)
    }

    pub fn update_placement(&mut self) -> Result<&mut Self> {
        let size = Popup::get_size();
        self.window.set_inner_size(size);
        let monitor = self.get_current_monitor()?;
        if let Some(monitor) = monitor {
            self.cursor_on_separate_monitor = false;
            self.window
                .set_outer_position(Popup::get_position(monitor, size));
        }
        Ok(self)
    }

    pub fn detect_cursor_monitor(&mut self) -> Result<&mut Self> {
        if let Some(cursor_monitor) = self.get_current_monitor()? {
            if let Some(window_monitor) = self.window.current_monitor() {
                self.cursor_on_separate_monitor = window_monitor.name() != cursor_monitor.name()
            }
        }
        Ok(self)
    }

    fn get_current_monitor(&self) -> Result<Option<MonitorHandle>> {
        if let Some((x, y)) = get_cursor_pos() {
            trace!("Found cursor position {:?}", (x, y));
            let monitor = self.window.monitor_from_point(x.into(), y.into());
            Ok(monitor)
        } else {
            Ok(None)
        }
    }

    fn get_position(
        monitor: MonitorHandle,
        window_size: PhysicalSize<i32>,
    ) -> PhysicalPosition<f32> {
        let monitor_position = monitor.position();
        let monitor_size = monitor.size();
        let [size_width, size_height] = [monitor_size.width, monitor_size.height].map(|n| n as f32);
        let [position_x, position_y, window_size_width, window_size_height] = [
            monitor_position.x,
            monitor_position.y,
            window_size.width,
            window_size.height,
        ]
        .map(|n| n as f32);
        let x: f32 = ((size_width + position_x) / 2.) - (window_size_width / 2.);
        let y: f32 = (position_y + size_height) - (window_size_height * 2.);
        trace!("Setting window position {:?}", (x, y));
        PhysicalPosition::new(x, y)
    }
}
