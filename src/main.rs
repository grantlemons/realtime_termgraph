use rt_termgraph::{char_at_position, write_to_row, Canvas};

fn main() {
    let _canvas = Canvas::default();
    let text: &str = "Hello from Grant!";
    write_to_row(4, text).unwrap();
    char_at_position(3, 0, '.').unwrap();
}
