use iced_native::{
    image, svg, Background, Color, Damage, Font, HorizontalAlignment,
    Rectangle, Size, Vector, VerticalAlignment,
};

use crate::triangle;
use std::sync::Arc;

/// A rendering primitive.
#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    /// An empty primitive
    None,
    /// A group of primitives
    Group {
        /// The primitives of the group
        primitives: Vec<Primitive>,
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
        content: Box<Primitive>,
    },
    /// A primitive that applies a translation
    Translate {
        /// The translation vector
        translation: Vector,

        /// The primitive to translate
        content: Box<Primitive>,
    },
    /// A low-level primitive to render a mesh of triangles.
    ///
    /// It can be used to render many kinds of geometry freely.
    Mesh2D {
        /// The vertex and index buffers of the mesh
        buffers: triangle::Mesh2D,

        /// The size of the drawable region of the mesh.
        ///
        /// Any geometry that falls out of this region will be clipped.
        size: Size,
    },
    /// A cached primitive.
    ///
    /// This can be useful if you are implementing a widget where primitive
    /// generation is expensive.
    Cached {
        /// The cached primitive
        cache: Arc<Primitive>,
    },
}

impl Default for Primitive {
    fn default() -> Primitive {
        Primitive::None
    }
}

impl Primitive {
    fn bounds(&self) -> Option<Rectangle> {
        match self {
            Primitive::Quad { bounds, .. }
            | Primitive::Image { bounds, .. }
            | Primitive::Svg { bounds, .. }
            | Primitive::Clip { bounds, .. } => Some(bounds.clone()),
            Primitive::Text {
                bounds,
                vertical_alignment,
                horizontal_alignment,
                ..
            } => Some(Rectangle {
                x: match horizontal_alignment {
                    HorizontalAlignment::Left => bounds.x,
                    HorizontalAlignment::Center => {
                        bounds.x - bounds.width / 2.0
                    }
                    HorizontalAlignment::Right => bounds.x - bounds.width,
                },
                y: match vertical_alignment {
                    VerticalAlignment::Top => bounds.y,
                    VerticalAlignment::Center => bounds.y - bounds.height / 2.0,
                    VerticalAlignment::Bottom => bounds.y - bounds.height,
                },
                ..*bounds
            }),
            Primitive::Group { primitives } => {
                let mut iter = primitives.iter().flat_map(|a| a.bounds());
                let first = iter.next()?;
                Some(iter.fold(first, |a, b| a.union(&b)))
            }
            _ => None,
        }
    }
}

impl Damage for Primitive {
    fn damage(&self, other: &Primitive) -> Option<Vec<Rectangle>> {
        if let (
            Primitive::Cached { cache: lcache },
            Primitive::Cached { cache: rcache },
        ) = (self, other)
        {
            return lcache.as_ref().damage(rcache.as_ref());
        }
        if let (
            Primitive::Group { primitives: lprims },
            Primitive::Group { primitives: rprims },
        ) = (self, other)
        {
            if lprims.len() == rprims.len() {
                return Some(
                    lprims
                        .iter()
                        .zip(rprims.iter())
                        .flat_map(|(lp, rp)| lp.damage(rp))
                        .flatten()
                        .collect(),
                );
            }
        }
        if self != other {
            match (self.bounds(), other.bounds()) {
                (Some(lb), Some(rb)) if lb != rb => Some(vec![lb, rb]),
                (Some(lb), _) => Some(vec![lb]),
                (_, Some(rb)) => Some(vec![rb]),
                _ => None,
            }
        } else {
            None
        }
    }
}
