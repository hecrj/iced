//! Display fields that can be filled with text.
//!
//! A [`TextInput`] has some local [`State`].
//!
//! [`TextInput`]: struct.TextInput.html
//! [`State`]: struct.State.html
use crate::{
    event::{EventHandler, WidgetEvent},
    Element, Hasher, Length, Widget,
    widget::{
        WidgetType,
        WidgetNode,
    },
};

pub use iced_style::text_input::{Style, StyleSheet};

use std::{
    ffi::CStr,
    rc::Rc,
    u32
};

use uikit_sys::{
    id, INSNotificationCenter, INSObject, IUITextView,
    NSNotificationCenter, NSString, NSString_NSStringExtensionMethods,
    UITextView, UITextViewTextDidChangeNotification, UIView, IUIView,
    UIView_UIViewHierarchy,
    UIView_UIViewGeometry,
    ICALayer,
    UIScreen, IUIScreen,
};
#[allow(missing_debug_implementations)]
pub struct TextInput<'a, Message> {
    _state: &'a mut State,
    placeholder: String,
    value: String,
    is_secure: bool,
    width: Length,
    max_width: u32,
    padding: u16,
    size: Option<u16>,
    on_change: Rc<Box<dyn Fn(String) -> Message>>,
    on_submit: Option<Message>,
    style_sheet: Box<dyn StyleSheet>,
}

impl<'a, Message> TextInput<'a, Message> {
    /// Creates a new [`TextInput`].
    ///
    /// It expects:
    /// - some [`State`]
    /// - a placeholder
    /// - the current value
    /// - a function that produces a message when the [`TextInput`] changes
    ///
    /// [`TextInput`]: struct.TextInput.html
    /// [`State`]: struct.State.html
    pub fn new<F>(
        state: &'a mut State,
        placeholder: &str,
        value: &str,
        on_change: F,
    ) -> Self
    where
        F: 'static + Fn(String) -> Message,
    {
        debug!("CREATING NEW TEXT INPUT");
        Self {
            _state: state,
            placeholder: String::from(placeholder),
            value: String::from(value),
            is_secure: false,
            width: Length::Fill,
            max_width: u32::MAX,
            padding: 0,
            size: None,
            on_change: Rc::new(Box::new(on_change)),
            on_submit: None,
            style_sheet: Default::default(),
        }
    }

    /// Converts the [`TextInput`] into a secure password input.
    ///
    /// [`TextInput`]: struct.TextInput.html
    pub fn password(mut self) -> Self {
        self.is_secure = true;
        self
    }

    /// Sets the width of the [`TextInput`].
    ///
    /// [`TextInput`]: struct.TextInput.html
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the maximum width of the [`TextInput`].
    ///
    /// [`TextInput`]: struct.TextInput.html
    pub fn max_width(mut self, max_width: u32) -> Self {
        self.max_width = max_width;
        self
    }

    /// Sets the padding of the [`TextInput`].
    ///
    /// [`TextInput`]: struct.TextInput.html
    pub fn padding(mut self, units: u16) -> Self {
        self.padding = units;
        self
    }

    /// Sets the text size of the [`TextInput`].
    ///
    /// [`TextInput`]: struct.TextInput.html
    pub fn size(mut self, size: u16) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets the message that should be produced when the [`TextInput`] is
    /// focused and the enter key is pressed.
    ///
    /// [`TextInput`]: struct.TextInput.html
    pub fn on_submit(mut self, message: Message) -> Self {
        self.on_submit = Some(message);
        self
    }

    /// Sets the style of the [`TextInput`].
    ///
    /// [`TextInput`]: struct.TextInput.html
    pub fn style(mut self, style: impl Into<Box<dyn StyleSheet>>) -> Self {
        self.style_sheet = style.into();
        self
    }
}

impl<'a, Message> Widget<Message> for TextInput<'a, Message>
where
    Message: 'static + Clone,
{
    fn hash_layout(&self, state: &mut Hasher) {
        use std::{any::TypeId, hash::Hash};
        struct Marker;
        TypeId::of::<Marker>().hash(state);

        self.width.hash(state);
        self.max_width.hash(state);
        self.padding.hash(state);
        self.size.hash(state);
    }

    fn get_widget_type(&self) -> WidgetType {
        WidgetType::TextInput
    }
    fn update(&self, current_node: &mut WidgetNode, root_view: Option<UIView>) {
        match &current_node.widget_type {
            WidgetType::TextInput => {
                // TODO: check/update the styles of the input box.
            },
            other => {
                debug!("Updating from {:?}, to {:?}", other, self.get_widget_type());
                current_node.drop_from_ui();
                let new_node = self.build_uiview(root_view.is_some());
                if let Some(root_view) = root_view {
                    new_node.draw(root_view);
                }
                *current_node = new_node;

            }
        }
    }

    fn build_uiview(&self, is_root: bool) -> WidgetNode {
        info!("Building text input uiview");
        let mut ids_to_drop : Vec<id> = Vec::new();
        let textview = {
            let ui_textview = {
                let view = unsafe {UITextView(UITextView::alloc().init())};
                if is_root {
                    unsafe {
                        let frame = UIScreen::mainScreen().bounds();
                        view.setFrame_(frame);
                    }
                }
                view
            };

            let on_change = EventHandler::new(ui_textview.0);
            // https://developer.apple.com/documentation/foundation/nsnotificationcenter/1415360-addobserver?language=objc
            unsafe {
                NSNotificationCenter::defaultCenter().addObserver_selector_name_object_(
                    on_change.id,
                    sel!(sendEvent),
                    UITextViewTextDidChangeNotification.clone(),
                    ui_textview.0,
                );
            }
            ids_to_drop.push(on_change.id);
            // TODO: Make this a debug feature
            unsafe { ui_textview.layer().setBorderWidth_(3.0); }
            ui_textview
        };


        let mut node = WidgetNode::new(
            textview.0,
            self.get_widget_type(),
            self.get_my_hash()
        );
        for i in &ids_to_drop {
            node.add_related_id(*i);
        }
        node
    }

    fn on_widget_event(
        &mut self,
        widget_event: WidgetEvent,
        messages: &mut Vec<Message>,
        widget_node: &WidgetNode,
    ) {
        trace!(
            "on_widget_event for text input: widget_event.id: {:x} for widget_id: {:?}, widget_node.view_id {:?}",
            widget_event.id,
            widget_event.widget_id,
            widget_node.view_id,
            );
        if widget_event.id as id == widget_node.view_id {
            let ui_textview = UITextView(widget_event.id as id);
            let value = {
                // This is only unsafe due to the FFI of bindgen.
                // This copies the NSString and so we will be taking ownership of it.
                let value = unsafe {ui_textview.text()};

                // The documentation on weather this is nullable is unclear. Best to check anyway.
                if value.0 == 0 as id {
                    return;
                }
                let bytes = unsafe { value.UTF8String() };
                if bytes.is_null() {
                    return;
                }
                let cstr = unsafe { CStr::from_ptr(bytes) };
                cstr.to_string_lossy().to_owned().to_string()
            };
            if value.ends_with("\n") {
                if let Some(on_submit) = self.on_submit.take() {
                    messages.push(on_submit);
                }
            } else {
                messages.push((self.on_change)(self.value.clone()));
                self.value = value;
            }
        }
    }

    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        Length::Shrink
    }
}

impl<'a, Message> From<TextInput<'a, Message>> for Element<'a, Message>
where
    Message: 'static + Clone,
{
    fn from(text_input: TextInput<'a, Message>) -> Element<'a, Message> {
        Element::new(text_input)
    }
}

/// The state of a [`TextInput`].
///
/// [`TextInput`]: struct.TextInput.html
#[derive(Debug, Clone, Copy, Default)]
pub struct State;

impl State {
    /// Creates a new [`State`], representing an unfocused [`TextInput`].
    ///
    /// [`State`]: struct.State.html
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new [`State`], representing a focused [`TextInput`].
    ///
    /// [`State`]: struct.State.html
    pub fn focused() -> Self {
        // TODO
        Self::default()
    }
}