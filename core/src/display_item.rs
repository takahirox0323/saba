//! This is used from UI component.

use crate::renderer::layout::computed_style::ComputedStyle;
use crate::renderer::layout::layout_point::LayoutPoint;
use crate::renderer::layout::layout_size::LayoutSize;
use alloc::string::String;

#[derive(Debug, Clone, PartialEq)]
pub enum DisplayItem {
    Rect {
        style: ComputedStyle,
        layout_point: LayoutPoint,
        layout_size: LayoutSize,
    },
    Text {
        text: String,
        style: ComputedStyle,
        layout_point: LayoutPoint,
    },
    Img {
        src: String,
        style: ComputedStyle,
        layout_point: LayoutPoint,
    },
    Input {
        input_type: String,
        name: Option<String>,
        placeholder: Option<String>,
        value: Option<String>,
        style: ComputedStyle,
        layout_point: LayoutPoint,
        layout_size: LayoutSize,
    },
}

impl DisplayItem {
    pub fn is_rect(&self) -> bool {
        matches!(
            self,
            DisplayItem::Rect {
                style: _,
                layout_point: _,
                layout_size: _,
            }
        )
    }

    pub fn is_text(&self) -> bool {
        matches!(
            self,
            DisplayItem::Text {
                text: _,
                style: _,
                layout_point: _,
            }
        )
    }

    pub fn is_input(&self) -> bool {
        matches!(
            self,
            DisplayItem::Input {
                input_type: _,
                name: _,
                placeholder: _,
                value: _,
                style: _,
                layout_point: _,
                layout_size: _,
            }
        )
    }
}
