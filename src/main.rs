extern crate rand;
use rand::Rng;

extern crate draw_charts;

use draw_charts::timecop_data_transformer::DataPoint;

fn main() {
    let data = generate_timecop_data();
    let json = draw_charts::get_chart_json(&data, draw_charts::ScaleType::NONE);
    println!("Chart JSON {:?}", json);
//    draw_charts::draw_using_gust(&data, draw_charts::ScaleType::LOG2)
}

fn generate_timecop_data() -> Vec<DataPoint> {

    const POINTS_COUNT: i32 = 10;
    const START_TIME: i64 = 1529616142;
    let mut rng = rand::thread_rng();
    let data = (1..POINTS_COUNT)
        .map(|i| {
            DataPoint {
                id: 0,
                ts_ms: START_TIME + i as i64,
                sched_us: rng.gen_range(1, 50) as i64,
                mono_drift_usps: rng.gen_range(1, 150) as i64,
                wallclock_drift_usps: rng.gen_range(1, 300) as i64,
            }
        })
        .collect();
    println!("Data {:?}", data);
    return data;
}

fn generate_sample_data() -> Vec<Vec<i64>> {

    const LINES_COUNT: i32 = 4;
    let mut rng = rand::thread_rng();
    let data = (1..LINES_COUNT)
        .map(|_| {
            (0..20)
                .map(|_| {
                    rng.gen_range(5, 15) as i64
                })
                .collect()
        })
        .collect();
    return data;
}
