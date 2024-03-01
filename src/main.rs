use std::time::{Duration, Instant};

use plotters::prelude::*;
use rand::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("./out/0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..1f32, 0f32..1f32)?;

    chart.configure_mesh().draw()?;
    let mut durations: Vec<Duration> = Vec::new();
    // isize::max nyeheheh
    for n in 0..10_000 {
        println!("{n}");
        let nums: Vec<i32> = (0..n).collect();
        let mut _found = false;
        let start = Instant::now();
        for v in nums {
            if v == 1 {
                // do something
                _found = true;
            }
        }
        durations.push(start.elapsed());
    }
    let a = durations.iter().map(|d| d.as_micros()).collect::<Vec<_>>();

    let points = a.iter().enumerate().map(|(i, d)| {
        Circle::new(
            (
                i as f32 / 10_000.0,
                *d as f32 / *a.iter().max().unwrap() as f32,
            ),
            1,
            GREEN.filled(),
        )
    });
    chart.draw_series(points)?;

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}
