extern crate gust;
use gust::backend::line_chart::LineChart;
use gust::frontend::write::render_graph;
use gust::backend::general::FileType;
use gust::backend::traits::Graphable;

pub enum ScaleType {
    LOG2,
    LOG10,
    NONE,
}

pub fn draw_using_gust(data: &Vec<Vec<i64>>, scale: ScaleType) {

    let mut b = LineChart::new();

    for (l, numbers) in data.iter().enumerate() {
        for (i, n) in numbers.iter().enumerate() {
            // println!("In position {} we have value {}", i, n);
            b.add_data(i as i64, get_scaled_value(*n, &scale), l as i64);
        }
    }
    render_graph(&b, FileType::HTML).unwrap();
}

pub fn get_vega_chart_json(data: &Vec<Vec<i64>>, scale: ScaleType) -> String {

    let mut b = LineChart::new();

    for (l, numbers) in data.iter().enumerate() {
        for (i, n) in numbers.iter().enumerate() {
            b.add_data(i as i64, get_scaled_value(*n, &scale), l as i64);
        }
    }
    b.get_json_representation()
}

fn get_scaled_value(value: i64, scale: &ScaleType) -> i64 {
    match *scale {
        ScaleType::LOG2 => (value as f64).log2() as i64,
        ScaleType::LOG10 => (value as f64).log10() as i64,
        _ => value
    }
}