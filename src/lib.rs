mod lib {
    #[derive(Default, Clone, Copy)]
    pub struct Point(f32, f32);

    mod canvas;
    mod scatter_plot;

    pub use canvas::Canvas;
    pub use scatter_plot::ScatterPlot;
}
pub use lib::*;

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {}
}
