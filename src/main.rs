extern crate rand;
use rand::Rng;

extern crate draw_charts;

fn main() {
    let data = generate_data();
    draw_charts::draw_using_gust(&data, draw_charts::ScaleType::LOG2)
}

fn generate_data() -> Vec<Vec<i64>> {

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
