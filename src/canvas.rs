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

#[derive(Clone)]
pub struct Canvas {
    pub dimensions: (u16, u16),
    pub x_bounds: RangeInclusive<f32>,
    pub y_bounds: RangeInclusive<f32>,
    start_row: u16,
    marker: char,
    border_char: char,
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            dimensions: (20, 10),
            x_bounds: -10.0..=10.0,
            y_bounds: -10.0..=10.0,
            start_row: cursor::position().unwrap_or((0, 0)).1,
            marker: '.',
            border_char: '#',
        }
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        self.go_to_exit_pos();
        crossterm::execute!(stdout(), cursor::Show).unwrap();
    }
}

impl Canvas {
    pub fn new(rows: u16, columns: u16) -> Self {
        Self {
            dimensions: (rows, columns),
            x_bounds: -10.0..=10.0,
            y_bounds: -10.0..=10.0,
            ..Self::default()
        }
        .init()
    }

    pub fn custom(
        rows: u16,
        columns: u16,
        x_bounds: RangeInclusive<f32>,
        y_bounds: RangeInclusive<f32>,
        marker: char,
        border_char: char,
    ) -> Self {
        Self {
            dimensions: (rows, columns),
            x_bounds,
            y_bounds,
            marker,
            border_char,
            ..Self::default()
        }
        .init()
    }

    pub fn init(mut self) -> Self {
        Self::clear_rows(self.dimensions.1 + 4);
        crossterm::execute!(stdout(), cursor::MoveUp(self.dimensions.1 + 4)).unwrap();
        self.start_row = cursor::position().unwrap_or((0, 0)).1;
        self.print_border();

        self
    }

    pub fn style(mut self, marker: char, border_char: char) -> Self {
        self.marker = marker;
        self.border_char = border_char;
        self
    }

    pub fn x_bounds(mut self, bounds: RangeInclusive<f32>) -> Self {
        self.x_bounds = bounds;
        self
    }

    pub fn y_bounds(mut self, bounds: RangeInclusive<f32>) -> Self {
        self.y_bounds = bounds;
        self
    }

    pub fn bounds(self, x_bounds: RangeInclusive<f32>, y_bounds: RangeInclusive<f32>) -> Self {
        self.x_bounds(x_bounds).y_bounds(y_bounds)
    }

    pub fn plot_points(&self, points: &[Point]) {
        for point in points {
            self.plot_point(point);
        }
    }

    pub fn plot_point(&self, point: &Point) {
        if let Some((x, y)) = self.map_point(point) {
            self.char_at_position(y, x, self.marker);
        }
    }

    fn map_point(&self, point: &Point) -> Option<(u16, u16)> {
        let (x, y) = point;
        if self.x_bounds.contains(x) && self.y_bounds.contains(y) {
            let (xs, xe) = (self.x_bounds.start(), self.x_bounds.end());
            let (ys, ye) = (self.y_bounds.start(), self.y_bounds.end());

            let nx = x.map_range(*xs..*xe, 0.0..(self.dimensions.0 as f32));
            let ny = y.map_range(*ys..*ye, (self.dimensions.1 as f32)..0.0);

            return Some((nx.round() as u16, ny.round() as u16));
        }

        None
    }

    fn final_row(&self) -> u16 {
        self.start_row + self.dimensions.1 - 1
    }

    fn go_to_exit_pos(&self) {
        crossterm::execute!(stdout(), cursor::MoveTo(0, self.final_row() + 4)).unwrap();
    }

    fn clear_rows(count: u16) {
        (0..count).for_each(|_| println!());
    }

    pub fn print_border(&self) {
        let border_text = (0..=self.dimensions.0 + 2).map(|_| '#').collect::<String>();

        self.write_to_row(0, &border_text);
        self.write_to_row(self.dimensions.1 + 2, &border_text);

        for row in 1..=self.dimensions.1 + 1 {
            char_at_position(self.start_row + row, 0, self.border_char).unwrap();
            char_at_position(self.start_row + row, self.dimensions.0 + 2, '#').unwrap();
        }
        self.go_to_exit_pos();
    }

    fn write_to_row(&self, row: u16, text: &str) {
        write_to_row(self.start_row + row, text).unwrap();
        self.go_to_exit_pos();
    }

    fn char_at_position(&self, row: u16, column: u16, char: char) {
        char_at_position(self.start_row + row + 1, column + 1, char).unwrap();
        self.go_to_exit_pos();
    }
}

fn write_to_row(row: u16, text: &str) -> Result<(), std::io::Error> {
    let mut stdout = stdout();
    stdout.execute(cursor::Hide)?;

    stdout.queue(cursor::MoveTo(0, row))?;

    stdout.queue(Clear(ClearType::CurrentLine))?;
    stdout.write_all(text.as_bytes())?;

    stdout.flush()?;

    Ok(())
}

fn char_at_position(row: u16, column: u16, char: char) -> Result<(), std::io::Error> {
    let mut stdout = stdout();
    stdout.execute(cursor::Hide)?;

    stdout.queue(cursor::MoveTo(column, row))?;

    stdout.write_all(&[char as u8])?;

    stdout.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

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
            ((0.0, 0.0), (10, 5)),
            ((-10.0, 0.0), (0, 5)),
            ((10.0, 0.0), (20, 5)),
            ((-10.0, 10.0), (0, 0)),
            ((10.0, -10.0), (20, 10)),
        ];

        for (point, map) in expected_maps {
            assert_eq!(canvas.map_point(&point), Some(map));
        }
    }

    #[test]
    fn test_map_point_dimensions() {
        let canvas: Canvas = Canvas::new(40, 10);
        let expected_maps = [
            ((0.0, 0.0), (20, 5)),
            ((-10.0, 0.0), (0, 5)),
            ((10.0, 0.0), (40, 5)),
            ((-10.0, 10.0), (0, 0)),
            ((10.0, -10.0), (40, 10)),
        ];

        for (point, map) in expected_maps {
            assert_eq!(canvas.map_point(&point), Some(map));
        }
    }

    #[test]
    fn test_map_point_range() {
        let canvas = Canvas::custom(20, 10, 0.0..=40.0, 0.0..=20.0, '+', '#');
        let expected_maps = [((0.0, 10.0), (0, 5)), ((10.0, 0.0), (5, 10))];

        for (point, map) in expected_maps {
            assert_eq!(canvas.map_point(&point), Some(map));
        }
    }
}
