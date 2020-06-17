use crate::{
    Layout,
};
pub mod container;
pub mod checkbox;
pub mod text;
pub mod text_input;

pub use container::Container;
pub use checkbox::Checkbox;
pub use text::Text;
pub use text_input::TextInput;
use uikit_sys::UIView;

pub trait Widget<Message> {
    fn draw(
        &self,
        _parent: UIView,
    ) {
    }
}

#[allow(missing_debug_implementations)]
pub struct Element<'a, Message> {
    pub(crate) widget: Box<dyn Widget<Message> + 'a>,
}
impl<'a, Message> Element<'a, Message> {
    /// Create a new [`Element`] containing the given [`Widget`].
    ///
    /// [`Element`]: struct.Element.html
    /// [`Widget`]: widget/trait.Widget.html
    pub fn new(widget: impl Widget<Message> + 'a) -> Self {
        Self {
            widget: Box::new(widget),
        }
    }
}

