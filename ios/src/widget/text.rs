
use crate::{
//    css, Bus, Color, Css,
    Element, Font, HorizontalAlignment, Length, Color,
    VerticalAlignment, Widget,
};

/// A paragraph of text.
///
/// # Example
///
/// ```
/// # use iced_web::Text;
///
/// Text::new("I <3 iced!")
///     .size(40);
/// ```
#[derive(Debug, Clone)]
pub struct Text {
    content: String,
    size: Option<u16>,
    color: Option<Color>,
    font: Font,
    width: Length,
    height: Length,
    horizontal_alignment: HorizontalAlignment,
    vertical_alignment: VerticalAlignment,
}

impl Text {
    /// Create a new fragment of [`Text`] with the given contents.
    ///
    /// [`Text`]: struct.Text.html
    pub fn new<T: Into<String>>(label: T) -> Self {
        Text {
            content: label.into(),
            size: None,
            color: None,
            font: Font::Default,
            width: Length::Shrink,
            height: Length::Shrink,
            horizontal_alignment: HorizontalAlignment::Left,
            vertical_alignment: VerticalAlignment::Top,
        }
    }

    /// Sets the size of the [`Text`].
    ///
    /// [`Text`]: struct.Text.html
    pub fn size(mut self, size: u16) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets the [`Color`] of the [`Text`].
    ///
    /// [`Text`]: struct.Text.html
    /// [`Color`]: ../../struct.Color.html
    pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Sets the [`Font`] of the [`Text`].
    ///
    /// [`Text`]: struct.Text.html
    /// [`Font`]: ../../struct.Font.html
    pub fn font(mut self, font: Font) -> Self {
        self.font = font;
        self
    }

    /// Sets the width of the [`Text`] boundaries.
    ///
    /// [`Text`]: struct.Text.html
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Text`] boundaries.
    ///
    /// [`Text`]: struct.Text.html
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the [`HorizontalAlignment`] of the [`Text`].
    ///
    /// [`Text`]: struct.Text.html
    /// [`HorizontalAlignment`]: enum.HorizontalAlignment.html
    pub fn horizontal_alignment(
        mut self,
        alignment: HorizontalAlignment,
    ) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    /// Sets the [`VerticalAlignment`] of the [`Text`].
    ///
    /// [`Text`]: struct.Text.html
    /// [`VerticalAlignment`]: enum.VerticalAlignment.html
    pub fn vertical_alignment(mut self, alignment: VerticalAlignment) -> Self {
        self.vertical_alignment = alignment;
        self
    }
}

impl<'a, Message> Widget<Message> for Text {
}

impl<'a, Message> From<Text> for Element<'a, Message> {
    fn from(text: Text) -> Element<'a, Message> {
        Element::new(text)
    }
}
