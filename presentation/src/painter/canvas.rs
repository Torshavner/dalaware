use iced::widget::canvas::{Cache, Event, Geometry, Path, Program};
use iced::{mouse, Color, Point, Rectangle, Renderer, Size, Theme};

use super::message::Message;

pub struct PixelCanvas<'a> {
    pub pixels: &'a [u8; 28 * 28],
    pub cache: &'a Cache,
}

impl<'a> Program<Message> for PixelCanvas<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            let pixel_size = bounds.width / 28.0;

            for y in 0..28 {
                for x in 0..28 {
                    let pixel_value = self.pixels[y * 28 + x];

                    let gray = f32::from(pixel_value) / 255.0;
                    let color = Color::from_rgb(gray, gray, gray);

                    // Add 1px spacing between tiles for better visibility
                    let spacing = 1.0;
                    let tile_size = pixel_size - spacing;

                    let rect = Path::rectangle(
                        Point::new(x as f32 * pixel_size, y as f32 * pixel_size),
                        Size::new(tile_size, tile_size),
                    );

                    frame.fill(&rect, color);
                }
            }

            frame.stroke(
                &Path::rectangle(Point::ORIGIN, bounds.size()),
                iced::widget::canvas::Stroke::default()
                    .with_color(Color::from_rgb(0.5, 0.5, 0.5))
                    .with_width(2.0),
            );
        });

        vec![geometry]
    }

    fn update(
        &self,
        _state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: iced::mouse::Cursor,
    ) -> (iced::widget::canvas::event::Status, Option<Message>) {
        if let Some(position) = cursor.position_in(bounds) {
            let pixel_size = bounds.width / 28.0;
            let x = (position.x / pixel_size) as usize;
            let y = (position.y / pixel_size) as usize;

            match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    return (
                        iced::widget::canvas::event::Status::Captured,
                        Some(Message::CanvasMouseDown { x, y }),
                    );
                }
                Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                    return (
                        iced::widget::canvas::event::Status::Captured,
                        Some(Message::CanvasMouseMove { x, y }),
                    );
                }
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    return (
                        iced::widget::canvas::event::Status::Captured,
                        Some(Message::CanvasMouseUp),
                    );
                }
                _ => {}
            }
        }

        (iced::widget::canvas::event::Status::Ignored, None)
    }
}
