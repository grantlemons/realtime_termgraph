use realtime_termgraph::*;

fn main() {
    let canvas = Canvas::new(100, 50)
        .bounds(-10.0..=10.0, -10.0..=10.0)
        .style('+', '#');

    Function::new(canvas, vec![|x| 6.0 * x.sin(), |_| 0.0]);

    // let (tx, rx) = std::sync::mpsc::channel();
    // let mut plot = ScatterPlot::new(canvas);

    // for (x, y) in [
    //     (0, 0),
    //     (100, 100),
    //     (-100, -100),
    //     (100, -100),
    //     (-100, 100),
    //     (15, 12),
    //     (-10, 11),
    //     (-13, 74),
    //     (-10, 10),
    //     (100, -20),
    //     (-15, 7),
    //     (17, 21),
    //     (-9, -79),
    //     (-14, 70),
    //     (-28, -34),
    //     (-96, -19),
    //     (-39, 15),
    // ] {
    //     plot.add_point((x as f32, y as f32));
    //     std::thread::sleep(Duration::from_millis(400));
    // }
}
