#[allow(unused_imports)]
use crossterm::*;

use super::Point;

#[derive(Default)]
pub struct Canvas;

impl Canvas {
    pub fn refresh(_points: &[Point]) {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
}
