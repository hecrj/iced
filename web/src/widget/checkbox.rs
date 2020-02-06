use crate::{style, Bus, Color, Element, Widget};

use dodrio::bumpalo;
use std::rc::Rc;

/// A box that can be checked.
///
/// # Example
///
/// ```
/// # use iced_web::Checkbox;
///
/// pub enum Message {
///     CheckboxToggled(bool),
/// }
///
/// let is_checked = true;
///
/// Checkbox::new(is_checked, "Toggle me!", Message::CheckboxToggled);
/// ```
///
/// ![Checkbox drawn by Coffee's renderer](https://github.com/hecrj/coffee/blob/bda9818f823dfcb8a7ad0ff4940b4d4b387b5208/images/ui/checkbox.png?raw=true)
#[allow(missing_debug_implementations)]
pub struct Checkbox<Message> {
    is_checked: bool,
    on_toggle: Rc<dyn Fn(bool) -> Message>,
    label: String,
    label_color: Option<Color>,
}

impl<Message> Checkbox<Message> {
    /// Creates a new [`Checkbox`].
    ///
    /// It expects:
    ///   * a boolean describing whether the [`Checkbox`] is checked or not
    ///   * the label of the [`Checkbox`]
    ///   * a function that will be called when the [`Checkbox`] is toggled. It
    ///     will receive the new state of the [`Checkbox`] and must produce a
    ///     `Message`.
    ///
    /// [`Checkbox`]: struct.Checkbox.html
    pub fn new<F>(is_checked: bool, label: &str, f: F) -> Self
    where
        F: 'static + Fn(bool) -> Message,
    {
        Checkbox {
            is_checked,
            on_toggle: Rc::new(f),
            label: String::from(label),
            label_color: None,
        }
    }

    /// Sets the color of the label of the [`Checkbox`].
    ///
    /// [`Checkbox`]: struct.Checkbox.html
    pub fn label_color<C: Into<Color>>(mut self, color: C) -> Self {
        self.label_color = Some(color.into());
        self
    }
}

impl<Message> Widget<Message> for Checkbox<Message>
where
    Message: 'static,
{
    fn node<'b>(
        &self,
        bump: &'b bumpalo::Bump,
        bus: &Bus<Message>,
        _style_sheet: &mut style::Sheet<'b>,
    ) -> dodrio::Node<'b> {
        use dodrio::builder::*;

        let checkbox_label = bumpalo::format!(in bump, "{}", self.label);

        let event_bus = bus.clone();
        let on_toggle = self.on_toggle.clone();
        let is_checked = self.is_checked;

        // TODO: Complete styling
        label(bump)
            .children(vec![
                input(bump)
                    .attr("type", "checkbox")
                    .bool_attr("checked", self.is_checked)
                    .on("click", move |_root, vdom, _event| {
                        let msg = on_toggle(!is_checked);
                        event_bus.publish(msg);

                        vdom.schedule_render();
                    })
                    .finish(),
                text(checkbox_label.into_bump_str()),
            ])
            .finish()
    }
}

impl<'a, Message> From<Checkbox<Message>> for Element<'a, Message>
where
    Message: 'static,
{
    fn from(checkbox: Checkbox<Message>) -> Element<'a, Message> {
        Element::new(checkbox)
    }
}
