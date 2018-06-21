extern crate rand;
use rand::Rng;

extern crate gust;
use gust::backend::line_chart::LineChart;
use gust::frontend::write::render_graph;
use gust::backend::general::FileType;

fn main() {
    draw_using_gust()
}

fn draw_using_gust() {
    
    let mut rng = rand::thread_rng();
    let mut b = LineChart::new();

    for l in 1..4 {
        let numbers: Vec<i64>
            = (0..20)
                .map(|_| {
                    rng.gen_range(5, 15)
                })
                .collect();

        for (i, n) in numbers.iter().enumerate() {
            // println!("In position {} we have value {}", i, n);
            b.add_data(i as i64, *n, l);
        }
    }
    render_graph(&b, FileType::HTML).unwrap();
}
