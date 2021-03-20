/// A buffer for short-term storage and transfer within and between
/// applications.
#[allow(missing_debug_implementations)]
pub struct Clipboard<'a> {
    state: State,
    window: &'a winit::window::Window,
}

enum State {
    Connected(window_clipboard::Clipboard),
    Unavailable,
}

impl<'a> Clipboard<'a> {
    /// Creates a new [`Clipboard`] for the given window.
    pub fn connect(window: &'a winit::window::Window) -> Clipboard<'a> {
        let state = window_clipboard::Clipboard::connect(window)
            .ok()
            .map(State::Connected)
            .unwrap_or(State::Unavailable);

        Clipboard { state, window }
    }

    /// Reads the current content of the [`Clipboard`] as text.
    pub fn read(&self) -> Option<String> {
        match &self.state {
            State::Connected(clipboard) => clipboard.read().ok(),
            State::Unavailable => None,
        }
    }

    /// Writes the given text contents to the [`Clipboard`].
    pub fn write(&mut self, contents: String) {
        match &mut self.state {
            State::Connected(clipboard) => match clipboard.write(contents) {
                Ok(()) => {}
                Err(error) => {
                    log::warn!("error writing to clipboard: {}", error)
                }
            },
            State::Unavailable => {}
        }
    }
}

impl<'a> iced_native::Clipboard for Clipboard<'a> {
    fn read(&self) -> Option<String> {
        self.read()
    }

    fn write(&mut self, contents: String) {
        self.write(contents)
    }

    fn set_ime_position(&self, position: iced_core::Point) {
        self.window
            .set_ime_position(winit::dpi::LogicalPosition::new(
                position.x, position.y,
            ));
    }
}
