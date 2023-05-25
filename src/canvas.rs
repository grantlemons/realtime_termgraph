#[allow(unused_imports)]
use crossterm::{
    cursor, terminal,
    terminal::{Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};
use map_range::MapRange;
use std::{
    io::{stdout, Write},
    ops::RangeInclusive,
};

use super::Point;

pub struct CanvasBuilder {
    dimensions: (u16, u16),
    x_bounds: Option<RangeInclusive<f32>>,
    y_bounds: Option<RangeInclusive<f32>>,
}

impl Default for CanvasBuilder {
    fn default() -> Self {
        Self {
            dimensions: (20, 10),
            x_bounds: None,
            y_bounds: None,
        }
    }
}

impl CanvasBuilder {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            dimensions: (width, height),
            ..Self::default()
        }
    }

    pub fn build(self) -> Option<Canvas> {
        if let (Some(x_bounds), Some(y_bounds)) = (self.x_bounds, self.y_bounds) {
            Some(Canvas {
                dimensions: self.dimensions,
                x_bounds,
                y_bounds,
                ..Canvas::default()
            })
        } else {
            None
        }
    }

    pub fn x_bounds(self, bounds: RangeInclusive<f32>) -> Self {
        Self {
            x_bounds: Some(bounds),
            ..self
        }
    }

    pub fn y_bounds(self, bounds: RangeInclusive<f32>) -> Self {
        Self {
            y_bounds: Some(bounds),
            ..self
        }
    }
}

pub struct Canvas {
    dimensions: (u16, u16),
    x_bounds: RangeInclusive<f32>,
    y_bounds: RangeInclusive<f32>,
    start_row: u16,
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            dimensions: (20, 10),
            x_bounds: -10.0..=10.0,
            y_bounds: -10.0..=10.0,
            start_row: cursor::position().unwrap().1,
        }
    }
}

impl Canvas {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            dimensions: (width, height),
            ..Self::default()
        }
    }

    pub fn x_bounds(&mut self, bounds: RangeInclusive<f32>) {
        self.x_bounds = bounds;
    }

    pub fn y_bounds(&mut self, bounds: RangeInclusive<f32>) {
        self.y_bounds = bounds;
    }

    pub fn bounds(&mut self, x_bounds: RangeInclusive<f32>, y_bounds: RangeInclusive<f32>) {
        self.x_bounds = x_bounds;
        self.y_bounds = y_bounds;
    }

    pub fn update(&self, points: &[Point]) {
        for point in points {
            self.plot_point(point);
        }
    }

    pub fn plot_point(&self, point: &Point) {
        if let Some((x, y)) = self.map_point(point) {
            char_at_position(self.start_row + y, x, '·').unwrap();
        }
    }

    pub fn map_point(&self, point: &Point) -> Option<(u16, u16)> {
        let (x, y) = point.into();
        if self.x_bounds.contains(&x) && self.y_bounds.contains(&y) {
            let (xs, xe) = (self.x_bounds.start(), self.x_bounds.end());
            let (ys, ye) = (self.y_bounds.start(), self.y_bounds.end());

            let nx = x.map_range(*xs..*xe, 0.0..(self.dimensions.0 as f32));
            let ny = y.map_range(*ys..*ye, (self.dimensions.1 as f32)..0.0);

            return Some((nx.round() as u16, ny.round() as u16));
        }

        None
    }
}

pub fn write_to_row(row: u16, text: &str) -> Result<(), std::io::Error> {
    let mut stdout = stdout();
    stdout.execute(cursor::Hide)?;

    stdout.queue(cursor::SavePosition)?;
    stdout.queue(cursor::MoveTo(0, row))?;

    stdout.queue(Clear(ClearType::CurrentLine))?;
    stdout.write_all(text.as_bytes())?;

    stdout.flush()?;

    stdout.execute(cursor::Show)?;

    Ok(())
}

pub fn char_at_position(row: u16, column: u16, char: char) -> Result<(), std::io::Error> {
    let mut stdout = stdout();
    stdout.execute(cursor::Hide)?;

    stdout.queue(cursor::SavePosition)?;
    stdout.queue(cursor::MoveTo(column, row))?;

    stdout.write_all(&[char as u8])?;

    stdout.flush()?;

    stdout.execute(cursor::Show)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::Point;

    #[ignore]
    #[test]
    fn test_write() -> Result<(), std::io::Error> {
        let text: &str = "Hello from Grant!";
        write_to_row(4, text)?;
        char_at_position(3, 0, '.')?;

        Ok(())
    }

    #[test]
    fn test_map_point_default() {
        let canvas: Canvas = Canvas::default();
        let expected_maps = [
            (Point::new(0.0, 0.0), (10, 5)),
            (Point::new(-10.0, 0.0), (0, 5)),
            (Point::new(10.0, 0.0), (20, 5)),
            (Point::new(-10.0, 10.0), (0, 0)),
            (Point::new(10.0, -10.0), (20, 10)),
        ];

        for (point, map) in expected_maps {
            assert_eq!(canvas.map_point(&point), Some(map));
        }
    }

    #[test]
    fn test_map_point_dimensions() {
        let canvas: Canvas = Canvas::new(40, 10);
        let expected_maps = [
            (Point::new(0.0, 0.0), (20, 5)),
            (Point::new(-10.0, 0.0), (0, 5)),
            (Point::new(10.0, 0.0), (40, 5)),
            (Point::new(-10.0, 10.0), (0, 0)),
            (Point::new(10.0, -10.0), (40, 10)),
        ];

        for (point, map) in expected_maps {
            assert_eq!(canvas.map_point(&point), Some(map));
        }
    }

    #[test]
    fn test_map_point_range() {
        let canvas: Canvas = CanvasBuilder::default()
            .x_bounds(0.0..=40.0)
            .y_bounds(0.0..=20.0)
            .build()
            .unwrap();
        let expected_maps = [
            (Point::new(0.0, 10.0), (0, 5)),
            (Point::new(10.0, 0.0), (5, 10)),
        ];

        for (point, map) in expected_maps {
            assert_eq!(canvas.map_point(&point), Some(map));
        }
    }
}
