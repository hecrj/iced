//! Find system fonts or use the built-in ones.
#[cfg(all(feature = "font-source", not(target_arch = "wasm32")))]
mod source;

#[cfg(all(feature = "font-source", not(target_arch = "wasm32")))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "font-source", not(target_arch = "wasm32")))))]
pub use source::Source;

#[cfg(all(feature = "font-source", not(target_arch = "wasm32")))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "font-source", not(target_arch = "wasm32")))))]
pub use font_kit::{
    error::SelectionError as LoadError, family_name::FamilyName as Family,
};

/// A built-in fallback font, for convenience.
#[cfg(feature = "font-fallback")]
#[cfg_attr(docsrs, doc(cfg(feature = "font-fallback")))]
pub const FALLBACK: &[u8] = include_bytes!("../fonts/Lato-Regular.ttf");

/// A built-in icon font, for convenience.
#[cfg(feature = "font-icons")]
#[cfg_attr(docsrs, doc(cfg(feature = "font-icons")))]
pub const ICONS: iced_native::Font = iced_native::Font::External {
    name: "iced_wgpu icons",
    bytes: include_bytes!("../fonts/Icons.ttf"),
};

/// The `char` representing a ✔ icon in the built-in [`ICONS`] font.
///
/// [`ICONS`]: const.ICONS.html
#[cfg(feature = "font-icons")]
#[cfg_attr(docsrs, doc(cfg(feature = "font-icons")))]
pub const CHECKMARK_ICON: char = '\u{F00C}';

/// The `char` representing a ▼ icon in the built-in [`ICONS`] font.
///
/// [`ICONS`]: const.ICONS.html
#[cfg(feature = "font-icons")]
pub const ARROW_DOWN_ICON: char = '\u{E800}';
