use std::time::Instant;

use iced::{
    mouse::Cursor,
    widget::canvas::{self, stroke, Path, Stroke},
    Color, Point, Rectangle, Renderer, Theme, Vector,
};

/// state type used for canvas drawing of the transport line
#[derive(Debug)]
pub struct ArrangerLine {
    /// unused clock taken from example
    now: Instant,
    /// cache which stores the canvas drawing elements
    system_cache: canvas::Cache,
    /// the current tick for the sequencer
    tick: f64,
}
impl ArrangerLine {
    /// create a new state
    pub fn new() -> ArrangerLine {
        let now = Instant::now();
        Self { now, system_cache: canvas::Cache::default(), tick: 0.0 }
    }

    /// update the canvas state
    pub fn update2(&mut self, now: Instant, tick: f64) {
        self.now = now;
        self.system_cache.clear();
        self.tick = tick;
    }
}

impl<Message> canvas::Program<Message> for ArrangerLine {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        let start_point = Point::new(50.0, 0.0);
        let line_length = 500.0;
        let target = Point::new(start_point.x, start_point.y + line_length);
        let line_cache = self.system_cache.draw(renderer, bounds.size(), |frame| {
            frame.translate(Vector::new(self.tick as f32 * 120.0, 0.0));

            let line = Path::line(start_point, target);
            frame.stroke(
                &line,
                Stroke {
                    style: stroke::Style::Solid(Color::from_rgba8(200, 240, 255, 0.5)),
                    width: 1.0,
                    ..Stroke::default()
                },
            );
        });

        vec![line_cache]
    }
}
