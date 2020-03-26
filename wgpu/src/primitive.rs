use iced_native::{
    image, svg, Background, Color, Depth, Font, HorizontalAlignment, Point,
    Rectangle, Vector, VerticalAlignment,
};

use crate::triangle;
use std::sync::Arc;

/// A rendering primitive and its relative depth
pub type Item = (Primitive, Depth);

/// A rendering primitive.
#[derive(Debug, Clone)]
pub enum Primitive {
    /// An empty primitive
    None,
    /// A group of primitives
    Group {
        /// The primitives of the group
        primitives: Vec<Item>,
    },
    /// A text primitive
    Text {
        /// The contents of the text
        content: String,
        /// The bounds of the text
        bounds: Rectangle,
        /// The color of the text
        color: Color,
        /// The size of the text
        size: f32,
        /// The font of the text
        font: Font,
        /// The horizontal alignment of the text
        horizontal_alignment: HorizontalAlignment,
        /// The vertical alignment of the text
        vertical_alignment: VerticalAlignment,
    },
    /// A quad primitive
    Quad {
        /// The bounds of the quad
        bounds: Rectangle,
        /// The background of the quad
        background: Background,
        /// The border radius of the quad
        border_radius: u16,
        /// The border width of the quad
        border_width: u16,
        /// The border color of the quad
        border_color: Color,
    },
    /// An image primitive
    Image {
        /// The handle of the image
        handle: image::Handle,
        /// The bounds of the image
        bounds: Rectangle,
    },
    /// An SVG primitive
    Svg {
        /// The path of the SVG file
        handle: svg::Handle,

        /// The bounds of the viewport
        bounds: Rectangle,
    },
    /// A clip primitive
    Clip {
        /// The bounds of the clip
        bounds: Rectangle,
        /// The offset transformation of the clip
        offset: Vector<u32>,
        /// The content of the clip
        content: Box<Item>,
    },
    /// A low-level primitive to render a mesh of triangles.
    ///
    /// It can be used to render many kinds of geometry freely.
    Mesh2D {
        /// The top-left coordinate of the mesh
        origin: Point,

        /// The vertex and index buffers of the mesh
        buffers: triangle::Mesh2D,
    },
    /// A cached primitive.
    ///
    /// This can be useful if you are implementing a widget where primitive
    /// generation is expensive.
    Cached {
        /// The origin of the coordinate system of the cached primitives
        origin: Point,

        /// The cached primitive
        cache: Arc<Item>,
    },
}

impl Default for Primitive {
    fn default() -> Primitive {
        Primitive::None
    }
}
