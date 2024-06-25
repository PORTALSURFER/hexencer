use iced::{
    advanced::{renderer, Overlay},
    widget::container,
};

pub struct TickCursor<Message, Theme, Renderer> {}

impl<Message, Theme, Renderer> Overlay<Message, Theme, Renderer>
    for TickCursor<Message, Theme, Renderer>
where
    Message: Clone,
    Theme: Catalog,
    Renderer: renderer::Renderer,
{
    fn layout(&mut self, renderer: &Renderer, bounds: iced::Size) -> iced::advanced::layout::Node {}

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
    ) {
        let style = theme.style(self.class);

        container::draw_background(renderer, &style, layout.bounds());

        let defaults = renderer::Style {
            text_color: style.text_color.unwrap_or(inherited_style.text_color),
        };

        self.tooltip.as_widget().draw(
            self.state,
            renderer,
            theme,
            &defaults,
            layout.children().next().unwrap(),
            cursor_position,
            &Rectangle::with_size(Size::INFINITY),
        );
    }

    fn overlay<'a>(
        &'a mut self,
        _layout: iced::advanced::Layout<'_>,
        _renderer: &Renderer,
    ) -> Option<iced::advanced::overlay::Element<'a, Message, Theme, Renderer>> {
    }

    fn on_event(
        &mut self,
        _event: iced::Event,
        _layout: iced::advanced::Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::Clipboard,
        _shell: &mut iced::Shell<'_, Message>,
    ) -> iced::event::Status {
        iced::event::Status::Ignored
    }
}
