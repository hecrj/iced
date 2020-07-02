use crate::{
    button, checkbox, column, progress_bar, radio, row, scrollable, slider,
    text, text_input, Color, Element, Font, HorizontalAlignment, Layout, Point,
    Rectangle, Renderer, Size, VerticalAlignment,
};

/// A renderer that does nothing.
///
/// It can be useful if you are writing tests!
#[derive(Debug, Clone, Copy)]
pub struct Null;

impl Null {
    /// Creates a new [`Null`] renderer.
    ///
    /// [`Null`]: struct.Null.html
    pub fn new() -> Null {
        Null
    }
}

impl Renderer for Null {
    type Output = ();
    type Defaults = ();
}

impl column::Renderer for Null {
    fn draw<Message>(
        &mut self,
        _defaults: &Self::Defaults,
        _content: &[Element<'_, Message, Self>],
        _layout: Layout<'_>,
        _cursor_position: Point,
    ) {
    }
}

impl row::Renderer for Null {
    fn draw<Message>(
        &mut self,
        _defaults: &Self::Defaults,
        _content: &[Element<'_, Message, Self>],
        _layout: Layout<'_>,
        _cursor_position: Point,
    ) {
    }
}

impl text::Renderer for Null {
    type Font = Font;

    fn default_size(&self) -> u16 {
        20
    }

    fn measure(
        &self,
        _content: &str,
        _size: u16,
        _font: Font,
        _bounds: Size,
    ) -> (f32, f32) {
        (0.0, 20.0)
    }

    fn draw(
        &mut self,
        _defaults: &Self::Defaults,
        _bounds: Rectangle,
        _content: &str,
        _size: u16,
        _font: Font,
        _color: Option<Color>,
        _horizontal_alignment: HorizontalAlignment,
        _vertical_alignment: VerticalAlignment,
    ) {
    }
}

impl scrollable::Renderer for Null {
    type Style = ();

    fn scrollbar(
        &self,
        _bounds: Rectangle,
        _content_bounds: Rectangle,
        _offset: u32,
    ) -> Option<scrollable::Scrollbar> {
        None
    }

    fn draw(
        &mut self,
        _scrollable: &scrollable::State,
        _bounds: Rectangle,
        _content_bounds: Rectangle,
        _is_mouse_over: bool,
        _is_mouse_over_scrollbar: bool,
        _scrollbar: Option<scrollable::Scrollbar>,
        _offset: u32,
        _style: &Self::Style,
        _content: Self::Output,
    ) {
    }
}

impl text_input::Renderer for Null {
    type Font = Font;
    type Style = ();

    fn default_size(&self) -> u16 {
        20
    }

    fn measure_value(&self, _value: &str, _size: u16, _font: Font) -> f32 {
        0.0
    }

    fn offset(
        &self,
        _text_bounds: Rectangle,
        _font: Font,
        _size: u16,
        _value: &text_input::Value,
        _state: &text_input::State,
        _horizontal_alignment: HorizontalAlignment,
    ) -> f32 {
        0.0
    }

    fn draw(
        &mut self,
        _bounds: Rectangle,
        _text_bounds: Rectangle,
        _cursor_position: Point,
        _font: Font,
        _size: u16,
        _placeholder: &str,
        _value: &text_input::Value,
        _horizontal_alignment: HorizontalAlignment,
        _state: &text_input::State,
        _style: &Self::Style,
    ) -> Self::Output {
    }
}

impl button::Renderer for Null {
    const DEFAULT_PADDING: u16 = 0;

    type Style = ();

    fn draw<Message>(
        &mut self,
        _defaults: &Self::Defaults,
        _bounds: Rectangle,
        _cursor_position: Point,
        _is_disabled: bool,
        _is_pressed: bool,
        _style: &Self::Style,
        _content: &Element<'_, Message, Self>,
        _content_layout: Layout<'_>,
    ) -> Self::Output {
    }
}

impl radio::Renderer for Null {
    type Style = ();

    const DEFAULT_SIZE: u16 = 20;
    const DEFAULT_SPACING: u16 = 15;

    fn draw(
        &mut self,
        _bounds: Rectangle,
        _is_selected: bool,
        _is_mouse_over: bool,
        _label: Self::Output,
        _style: &Self::Style,
    ) {
    }
}

impl checkbox::Renderer for Null {
    type Style = ();

    const DEFAULT_SIZE: u16 = 20;
    const DEFAULT_SPACING: u16 = 15;

    fn draw(
        &mut self,
        _bounds: Rectangle,
        _is_checked: bool,
        _is_mouse_over: bool,
        _label: Self::Output,
        _style: &Self::Style,
    ) {
    }
}

impl slider::Renderer for Null {
    type Style = ();

    fn height(&self) -> u32 {
        30
    }

    fn draw(
        &mut self,
        _bounds: Rectangle,
        _cursor_position: Point,
        _range: std::ops::RangeInclusive<f32>,
        _value: f32,
        _is_dragging: bool,
        _style_sheet: &Self::Style,
    ) {
    }
}

impl progress_bar::Renderer for Null {
    type Style = ();

    const DEFAULT_HEIGHT: u16 = 30;

    fn draw(
        &self,
        _bounds: Rectangle,
        _range: std::ops::RangeInclusive<f32>,
        _value: f32,
        _style: &Self::Style,
    ) {
    }
}
