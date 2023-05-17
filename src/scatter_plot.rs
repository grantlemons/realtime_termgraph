use parking_lot::Mutex;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::thread;

use super::Canvas;
use super::Point;

#[derive(Default)]
pub struct ScatterPlot(Arc<Mutex<Vec<Point>>>);

impl ScatterPlot {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_stream(rx: Receiver<Point>) -> Self {
        let mut plot = Self::new();
        plot.add_stream(rx);

        plot
    }

    pub fn auto_refresh(&mut self) -> thread::JoinHandle<()> {
        const REFRESH_RATE_HZ: u64 = 120;

        let points_clone = Arc::clone(&self.0);

        thread::spawn(move || loop {
            let points = points_clone.lock();

            Canvas::refresh(&points);

            drop(points); // drop lock in order to prevent holding lock while waiting
            thread::sleep(std::time::Duration::from_millis(1000 / REFRESH_RATE_HZ));
        })
    }

    pub fn refresh(&mut self) {
        let points = self.0.lock();

        Canvas::refresh(&points);
    }

    pub fn add_point(&mut self, point: Point) {
        let mut points = self.0.lock();
        points.push(point);
    }

    pub fn add_stream(&mut self, rx: Receiver<Point>) -> thread::JoinHandle<()> {
        let points_clone = Arc::clone(&self.0);

        thread::spawn(move || loop {
            let mut points = points_clone.lock();

            if let Ok(point) = rx.recv() {
                points.push(point);
            }
        })
    }

    pub fn extend(&mut self, new_points: &[Point]) {
        let mut points = self.0.lock();
        points.extend(new_points.to_owned());
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
}
