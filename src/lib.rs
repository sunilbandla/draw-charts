extern crate gust;
use gust::backend::line_chart::LineChart;
use gust::frontend::write::render_graph;
use gust::backend::general::FileType;

pub fn draw_using_gust(data: &Vec<Vec<i64>>) {

    let mut b = LineChart::new();

    for (l, numbers) in data.iter().enumerate() {
        for (i, n) in numbers.iter().enumerate() {
            // println!("In position {} we have value {}", i, n);
            b.add_data(i as i64, *n, l as i64);
        }
    }
    render_graph(&b, FileType::HTML).unwrap();
}
