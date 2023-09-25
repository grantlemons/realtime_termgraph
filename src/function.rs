use super::Canvas;
use super::Point;

pub struct Function {
    points: Vec<Point>,
    canvas: Canvas,
    functions: Vec<fn(f32) -> f32>,
}

impl Function {
    pub fn new(canvas: Canvas, functions: Vec<fn(f32) -> f32>) -> Self {
        let mut new = Self {
            points: Vec::new(),
            canvas,
            functions,
        };
        let points = new.generate_points();
        new.canvas.plot_points(&points);
        new.points = points;

        new
    }

    fn generate_points(&self) -> Vec<Point> {
        const STEPS_PER_FLOAT_STEP: i16 = 5;
        let x_points = ((*self.canvas.x_bounds.start() as i16 * STEPS_PER_FLOAT_STEP)
            ..=(*self.canvas.x_bounds.end() as i16 * STEPS_PER_FLOAT_STEP))
            .map(|x| x as f32 / STEPS_PER_FLOAT_STEP as f32);

        self.functions
            .iter()
            .flat_map(|f| x_points.clone().zip(x_points.clone().map(f)))
            .collect()
    }
}
