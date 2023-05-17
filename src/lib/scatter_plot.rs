use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;

use super::Canvas;
use super::Point;

#[derive(Default)]
pub struct ScatterPlot {
    canvas: Arc<Canvas>,
    points: Arc<Mutex<Vec<Point>>>,
}

impl ScatterPlot {
    pub fn new(canvas: Canvas) -> Self {
        Self {
            canvas: Arc::new(canvas),
            points: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn auto_update_graph(&mut self) -> thread::JoinHandle<()> {
        let canvas_clone = Arc::clone(&self.canvas);
        let points_clone = Arc::clone(&self.points);

        thread::spawn(move || {
            let points = points_clone.lock();

            Self::update_drawing(&canvas_clone, &points);

            drop(points); // drop lock in order to prevent holding lock while waiting
            thread::sleep(std::time::Duration::from_nanos(20));
        })
    }

    pub fn update_graph(&mut self) {
        let points = self.points.lock();

        Self::update_drawing(&self.canvas, &points);
    }

    #[allow(unused_variables)]
    fn update_drawing(canvas: &Canvas, points: &[Point]) {
        todo!();
    }

    pub fn add_point(&mut self, point: Point) {
        let mut points = self.points.lock();
        points.push(point);
    }

    pub fn append(&mut self, new_points: &mut Vec<Point>) {
        let mut points = self.points.lock();
        points.append(new_points);
    }

    pub fn extend(&mut self, new_points: &[Point]) {
        let mut points = self.points.lock();
        points.extend(new_points.to_owned());
    }
}
