#[derive(Debug, Default, Clone, Copy)]
pub struct Point(f32, f32);

impl<T: Into<f32>, U: Into<f32>> From<(T, U)> for Point {
    fn from(value: (T, U)) -> Self {
        Self(value.0.into(), value.1.into())
    }
}

impl<T: From<f32>, U: From<f32>> From<&Point> for (T, U) {
    fn from(value: &Point) -> Self {
        (value.0.into(), value.1.into())
    }
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point(x, y)
    }
}

impl Point {
    pub fn x(&self) -> f32 {
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }
}
