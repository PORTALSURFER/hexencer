use iced::{Point, Rectangle};

use super::Alignment;

#[derive(Debug, Copy, Clone)]
pub struct Scrollbar {
    pub total_bounds: Rectangle,
    pub bounds: Rectangle,
    pub scroll_handle: ScrollHandle,
    pub alignment: Alignment,
}

impl Scrollbar {
    /// Returns whether the mouse is over the scrollbar or not.
    pub fn is_mouse_over(&self, cursor_position: Point) -> bool {
        self.total_bounds.contains(cursor_position)
    }

    /// Returns the x-axis scrolled percentage from the cursor position.
    pub fn scroll_percentage(&self, grabbed_at: f32, cursor_position: Point) -> f32 {
        let percentage =
            (cursor_position.x - self.bounds.x - self.scroll_handle.bounds.width * grabbed_at)
                / (self.bounds.width - self.scroll_handle.bounds.width);

        match self.alignment {
            Alignment::Start => percentage,
            Alignment::_End => 1.0 - percentage,
        }
    }
}

/// The handle of a [`Scrollbar`].
#[derive(Debug, Clone, Copy)]
pub struct ScrollHandle {
    /// The bounds of the [`Scroller`].
    pub bounds: Rectangle,
}
