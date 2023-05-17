#[derive(Default, Clone, Copy)]
pub struct Point(f32, f32);

mod canvas;
mod scatter_plot;

pub use canvas::Canvas;
pub use scatter_plot::ScatterPlot;
