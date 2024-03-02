use std::time::Instant;

use plotters::prelude::*;
use rand::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let max = 1000;
    let mut durations: Vec<f32> = Vec::new();
    // isize::max nyeheheh
    for n in 0..max {
        // println!("{n}");
        let mut nums: Vec<i32> = (0..n).collect();
        let mut rng = rand::thread_rng();
        nums.shuffle(&mut rng);
        let instant = Instant::now();
        // alg here
        insertion_sort(&mut nums);
        durations.push(instant.elapsed().as_secs_f32());
    }

    let max_x = durations.len() as f32;
    let max_y = durations.clone().into_iter().reduce(f32::max).unwrap();

    let root = BitMapBackend::new("./out/0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("O(n)", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..max_x, 0f32..max_y)?;

    chart.configure_mesh().x_desc("Hello").draw()?;

    let points = durations
        .iter()
        .enumerate()
        .map(|(i, d)| Circle::new((i as f32, *d), 1, GREEN.filled()));
    chart.draw_series(points)?;

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}

fn insertion_sort(arr: &mut Vec<i32>) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::insertion_sort;

    #[test]
    fn test_insertion_sort() {
        let mut v = vec![5, 3, 4, 1, 2];
        insertion_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }
}
