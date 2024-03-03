use plotters::prelude::*;
use rand::prelude::*;
use std::{fs::create_dir, path::Path, time::Instant};
fn main() -> anyhow::Result<()> {
    println!("Welcome!");
    let path = Path::new("./out");
    if !path.exists() {
        println!("Output dir does not exist, creating it now.");
        create_dir(path)?;
    }
    println!("Measuring and graphing algorithms...");
    let algorithms = [insertion_sort, selection_sort];
    let names = ["insertion_sort", "selection_sort"];
    for (index, algorithm) in algorithms.iter().enumerate() {
        graph(
            algorithm,
            &path.join(format!("{}_{}.png", index, names[index])),
            1000,
        )?;
    }
    Ok(())
}

fn graph<F>(algorithm: F, path: &Path, max_elements: i32) -> anyhow::Result<()>
where
    F: Fn(&mut [i32]),
{
    let mut durations: Vec<f32> = Vec::new();
    // isize::max nyeheheh
    println!(" - measuring...");
    for n in 1..max_elements {
        // println!("{n}");
        let mut nums: Vec<i32> = (0..n).collect();
        let mut rng = rand::thread_rng();
        nums.shuffle(&mut rng);
        let instant = Instant::now();
        // alg here
        algorithm(&mut nums);
        durations.push(instant.elapsed().as_secs_f32());
    }
    println!(" - measurements completed. ");
    println!(" - graphing");
    // get some stuff set up for the graph
    let max_x = durations.len() as f32;
    let max_y = durations.clone().into_iter().reduce(f32::max).unwrap();
    let resolution = (640, 480);
    // configure graph
    let root = BitMapBackend::new(path, resolution).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Time Taken vs. Num. elements",
            ("sans-serif", 50).into_font(),
        )
        .margin(30)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..max_x, 0f32..max_y)?;
    chart
        .configure_mesh()
        .x_desc("Number of elements")
        .y_desc("Time elapsed (S)")
        .draw()?;

    // get points
    let points = durations
        .iter()
        .enumerate()
        .map(|(i, d)| Circle::new((i as f32, *d), 1, GREEN.filled()));

    // finalize graph; draw, present, etc.
    chart.draw_series(points)?;
    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;
    root.present()?;
    println!(" - graphing completed.");
    println!("Successfully measured data and created a graph!");
    Ok(())
}
fn insertion_sort(array: &mut [i32]) {
    for i in 1..array.len() {
        let mut j = i;
        while j > 0 && array[j] < array[j - 1] {
            array.swap(j, j - 1);
            j -= 1;
        }
    }
}
fn selection_sort(array: &mut [i32]) {
    for index in 0..array.len() - 1 {
        let mut minimum_index = index;
        for j in index + 1..array.len() {
            if array[j] < array[minimum_index] {
                minimum_index = j;
            }
        }
        if minimum_index != index {
            array.swap(minimum_index, index);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{insertion_sort, selection_sort};

    #[test]
    fn test_insertion_sort() {
        let mut v = vec![5, 3, 4, 1, 2];
        insertion_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }
    #[test]
    fn test_selection_sort() {
        let mut v = vec![5, 3, 4, 1, 2];
        selection_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }
}
