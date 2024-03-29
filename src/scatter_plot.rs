use super::Canvas;
use super::Point;

#[derive(Default)]
pub struct ScatterPlot {
    points: Vec<Point>,
    canvas: Canvas,
    callback: Option<fn(&Self)>,
}

impl ScatterPlot {
    pub fn new(canvas: Canvas) -> Self {
        Self {
            canvas,
            ..Self::default()
        }
    }

    pub fn new_with_points(canvas: Canvas, points: Vec<Point>) -> Self {
        Self {
            points,
            ..Self::new(canvas)
        }
    }

    pub fn new_with_callback(canvas: Canvas, func: fn(&Self)) -> Self {
        Self {
            callback: Some(func),
            ..Self::new(canvas)
        }
    }

    pub fn points(&self) -> Vec<Point> {
        self.points.clone()
    }

    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
        self.canvas.plot_point(&point);

        if let Some(callback) = self.callback {
            callback(self);
        }
    }

    pub fn extend(&mut self, new_points: &[Point]) {
        self.points.extend(new_points.to_owned());
        self.canvas.plot_points(new_points);

        if let Some(callback) = self.callback {
            callback(self);
        }
    }

    pub fn refresh(&self) {
        self.canvas.plot_points(&self.points);
    }
}

#[cfg(test)]
mod tests {
    // use crate::canvas::Canvas;

    #[allow(unused_imports)]
    use super::*;

    // #[test]
    // fn test_stream() {
    //     let (tx, rx) = std::sync::mpsc::channel();
    //     ScatterPlot::new_with_stream(Canvas::default(), rx);
    //
    //     for (x, y) in [(534, 234), (5423, 9856), (243342, 443), (2321, 43534)] {
    //         tx.send(Point::new(x as f32, y as f32)).unwrap();
    //     }
    // }
}
