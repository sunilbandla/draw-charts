extern crate gust;
extern crate serde;
extern crate serde_json;

use serde_json::{Value};
use gust::backend::line_chart::LineChart;
use gust::frontend::write::render_graph;
use gust::backend::general::FileType;
use gust::backend::traits::Graphable;

pub mod timecop_data_transformer;

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

pub fn get_chart_json(datapoints: &Vec<timecop_data_transformer::DataPoint>, scale: ScaleType) -> String {
    let mut b = LineChart::new();

    for (i, datapoint) in datapoints.iter().enumerate() {
        b.add_data(datapoint.ts_ms, get_scaled_value(datapoint.mono_drift_usps, &scale), 0);
        b.add_data(datapoint.ts_ms, get_scaled_value(datapoint.sched_us, &scale), 1);
        b.add_data(datapoint.ts_ms, get_scaled_value(datapoint.wallclock_drift_usps, &scale), 2);
    }
    let graph_json = b.get_json_representation();
    customise_chart_json(&graph_json)
}

pub fn get_vega_chart_json_for_sample_data(data: &Vec<Vec<i64>>, scale: ScaleType) -> String {

    let mut b = LineChart::new();

    for (l, numbers) in data.iter().enumerate() {
        for (i, n) in numbers.iter().enumerate() {
            b.add_data(i as i64, get_scaled_value(*n, &scale), l as i64);
        }
    }
    let graph_json = b.get_json_representation();
    customise_chart_json(&graph_json)
}

fn customise_chart_json(graph_json: &String) -> String {
    let json = convert_to_step_graph(graph_json);
    return json;
}

fn convert_to_step_graph(graph_json: &String) -> String {
    let mut value: Value = serde_json::from_str(&graph_json).unwrap();
    println!("old type: {:?}", value.pointer("/signals/0/value"));
    *value.pointer_mut("/signals/0/value").unwrap() = "step".into();
    println!("new type: {}", value["signals"][0]["value"]);
    value.to_string()
}

fn get_scaled_value(value: i64, scale: &ScaleType) -> i64 {
    match *scale {
        ScaleType::LOG2 => (value as f64).log2() as i64,
        ScaleType::LOG10 => (value as f64).log10() as i64,
        _ => value
    }
}