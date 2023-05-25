use parking_lot::Mutex;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::thread;

use super::Canvas;
use super::Point;

#[derive(Default)]
pub struct ScatterPlot {
    points: Arc<Mutex<Vec<Point>>>,
    canvas: Arc<Canvas>,
    stream_handle: Option<thread::JoinHandle<()>>,
}

// impl Drop for ScatterPlot {
//     fn drop(&mut self) {
//         if self.stream_handle.is_some() {
//             self.stream_handle.join().unwrap();
//         }
//         self.refresh()
//     }
// }

impl ScatterPlot {
    pub fn new(canvas: Canvas) -> Self {
        Self {
            canvas: Arc::new(canvas),
            ..Self::default()
        }
    }

    pub fn new_with_stream(canvas: Canvas, rx: Receiver<Point>) -> Self {
        let mut plot = Self::new(canvas);
        plot.add_stream(rx);

        plot
    }

    pub fn auto_refresh(&self) -> thread::JoinHandle<()> {
        use std::time::Duration;

        const REFRESH_RATE_HZ: u64 = 120;
        const REFRESH_PERIOD: Duration = Duration::from_millis(1000 / REFRESH_RATE_HZ);

        let points_clone = Arc::clone(&self.points);
        let canvas_clone = Arc::clone(&self.canvas);
        let mut last_count = 0;

        thread::spawn(move || loop {
            let points = points_clone.lock();

            let len = points.len();
            if points.len() != last_count {
                last_count = len;
                canvas_clone.update(&points);
            }

            drop(points); // drop lock in order to prevent holding lock while waiting
            thread::sleep(REFRESH_PERIOD);
        })
    }

    pub fn refresh(&self) {
        let points = self.points.lock();
        self.canvas.update(&points);
    }

    pub fn add_point(&mut self, point: Point) {
        let mut points = self.points.lock();
        points.push(point);
    }

    pub fn add_stream(&mut self, rx: Receiver<Point>) {
        let points_clone = Arc::clone(&self.points);

        let _handle = thread::spawn(move || {
            for point in rx {
                let mut points = points_clone.lock();
                points.push(point);
            }
        });

        // self.stream_handle = Some(handle);
    }

    pub fn extend(&mut self, new_points: &[Point]) {
        let mut points = self.points.lock();
        points.extend(new_points.to_owned());
    }
}

#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;

    #[allow(unused_imports)]
    use super::*;

    #[ignore]
    #[test]
    fn test_stream() {
        let (tx, rx) = std::sync::mpsc::channel();
        let plot = ScatterPlot::new_with_stream(Canvas::default(), rx);
        plot.auto_refresh();

        for (x, y) in [(534, 234), (5423, 9856), (243342, 443), (2321, 43534)] {
            tx.send(Point::new(x as f32, y as f32)).unwrap();
        }
    }
}
