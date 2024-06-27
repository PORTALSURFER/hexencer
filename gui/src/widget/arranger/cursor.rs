// use iced::{
//     advanced::{layout::Node, renderer, Overlay},
//     widget::container,
//     Element, Rectangle, Size,
// };

// use crate::widget;

// pub struct TickCursor<'a, 'b, Message, Theme, Renderer>
// where
//     Theme: container::Catalog,
//     Renderer: renderer::Renderer,
// {
//     tooltip: &'b Element<'a, Message, Theme, Renderer>,
//     state: &'b mut widget::Tree,
//     class: &'b Theme::Class<'a>,
// }

// impl<'a, Message, Theme, Renderer> TickCursor<'a, 'b, Message, Theme, Renderer>
// where
//     Theme: container::Catalog,
//     Renderer: renderer::Renderer,
// {
//     pub fn new(
//         content: impl Into<Element<'a, Message, Theme, Renderer>>,
//         tooltip: impl Into<Element<'a, Message, Theme, Renderer>>,
//     ) -> Self {
//         Self {
//             tooltip: tooltip.into(),
//             class: Theme::default(),
//         }
//     }
// }

// impl<'a, Message, Theme, Renderer> Overlay<Message, Theme, Renderer>
//     for TickCursor<'a, Message, Theme, Renderer>
// where
//     Theme: container::Catalog,
//     Renderer: renderer::Renderer,
// {
//     fn layout(&mut self, renderer: &Renderer, bounds: iced::Size) -> iced::advanced::layout::Node {
//         let size = iced::Size::new(100.0, 100.0);
//         Node::new(size)
//     }

//     fn draw(
//         &self,
//         renderer: &mut Renderer,
//         theme: &Theme,
//         inherited_style: &iced::advanced::renderer::Style,
//         layout: iced::advanced::Layout<'_>,
//         cursor: iced::advanced::mouse::Cursor,
//     ) {
//         let style = theme.style(self.class);

//         container::draw_background(renderer, &style, layout.bounds());

//         let defaults = renderer::Style {
//             text_color: style.text_color.unwrap_or(inherited_style.text_color),
//         };

//         self.tooltip.as_widget().draw(
//             self.state,
//             renderer,
//             theme,
//             &defaults,
//             layout.children().next().unwrap(),
//             cursor,
//             &Rectangle::with_size(Size::INFINITY),
//         );
//     }
// }
