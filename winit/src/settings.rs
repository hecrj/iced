//! Configure your application.
#[cfg(target_os = "windows")]
#[path = "settings/windows.rs"]
mod platform;
#[cfg(target_os = "macos")]
#[path = "settings/macos.rs"]
mod platform;
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
#[path = "settings/other.rs"]
mod platform;
pub use platform::PlatformSpecific;

use crate::conversion;
use crate::Mode;
use winit::monitor::MonitorHandle;
use winit::window::WindowBuilder;

/// The settings of an application.
#[derive(Debug, Clone, Default)]
pub struct Settings<Flags> {
    /// The [`Window`] settings
    pub window: Window,

    /// The data needed to initialize an [`Application`].
    ///
    /// [`Application`]: crate::Application
    pub flags: Flags,

    /// Whether the [`Application`] should exit when the user requests the
    /// window to close (e.g. the user presses the close button).
    pub exit_on_close_request: bool,
}

/// The window settings of an application.
#[derive(Debug, Clone)]
pub struct Window {
    /// The size of the window.
    pub size: (u32, u32),

    /// The minimum size of the window.
    pub min_size: Option<(u32, u32)>,

    /// The maximum size of the window.
    pub max_size: Option<(u32, u32)>,

    /// Whether the window should be resizable or not.
    pub resizable: bool,

    /// Whether the window should have a border, a title bar, etc.
    pub decorations: bool,

    /// Whether the window should be transparent.
    pub transparent: bool,

    /// Whether the window will always be on top of other windows.
    pub always_on_top: bool,

    /// The window icon, which is also usually used in the taskbar
    pub icon: Option<winit::window::Icon>,

    /// Platform specific settings.
    pub platform_specific: platform::PlatformSpecific,
}

impl Window {
    /// Converts the window settings into a `WindowBuilder` from `winit`.
    pub fn into_builder(
        self,
        title: &str,
        mode: Mode,
        primary_monitor: Option<MonitorHandle>,
    ) -> WindowBuilder {
        let mut window_builder = WindowBuilder::new();

        let (width, height) = self.size;

        window_builder = window_builder
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize { width, height })
            .with_resizable(self.resizable)
            .with_decorations(self.decorations)
            .with_transparent(self.transparent)
            .with_window_icon(self.icon)
            .with_always_on_top(self.always_on_top)
            .with_fullscreen(conversion::fullscreen(primary_monitor, mode))
            .with_visible(conversion::visible(mode));

        if let Some((width, height)) = self.min_size {
            window_builder = window_builder
                .with_min_inner_size(winit::dpi::LogicalSize { width, height });
        }

        if let Some((width, height)) = self.max_size {
            window_builder = window_builder
                .with_max_inner_size(winit::dpi::LogicalSize { width, height });
        }

        #[cfg(target_os = "windows")]
        {
            use winit::platform::windows::WindowBuilderExtWindows;

            if let Some(parent) = self.platform_specific.parent {
                window_builder = window_builder.with_parent_window(parent);
            }
            window_builder = window_builder
                .with_drag_and_drop(self.platform_specific.drag_and_drop);
        }

        #[cfg(target_os = "macos")]
        {
            use winit::platform::macos::WindowBuilderExtMacOS;

            window_builder = window_builder
                .with_title_hidden(self.platform_specific.title_hidden)
                .with_titlebar_transparent(
                    self.platform_specific.titlebar_transparent,
                )
                .with_fullsize_content_view(
                    self.platform_specific.fullsize_content_view,
                );
        }

        window_builder
    }
}

impl Default for Window {
    fn default() -> Window {
        Window {
            size: (1024, 768),
            min_size: None,
            max_size: None,
            resizable: true,
            decorations: true,
            transparent: false,
            always_on_top: false,
            icon: None,
            platform_specific: Default::default(),
        }
    }
}
