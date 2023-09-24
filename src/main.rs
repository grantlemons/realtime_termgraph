use rt_termgraph::*;

fn main() {
    use std::time::Duration;

    let canvas = Canvas::new(100, 50)
        .bounds(-100.0..=100.0, -100.0..=100.0)
        .style('+', '#');

    canvas.plot_point(&Point::new(0.0, 0.0));
    canvas.plot_point(&Point::new(-100.0, 100.0));
    canvas.plot_point(&Point::new(100.0, -100.0));
    canvas.plot_point(&Point::new(-100.0, -100.0));
    canvas.plot_point(&Point::new(100.0, 100.0));

    let (tx, rx) = std::sync::mpsc::channel();
    let (_, _) = ScatterPlot::new_with_stream(canvas, rx);

    for (x, y) in [
        (15, 12),
        (-10, 11),
        (-13, 74),
        (-10, 10),
        (100, -20),
        (-15, 7),
        (17, 21),
        (-9, -79),
        (-14, 70),
        (-12, -17),
        (96, 19),
        (-39, 15),
    ] {
        tx.send(Point::new(x as f32, y as f32)).unwrap();
        std::thread::sleep(Duration::from_millis(700));
    }
}
