// Function to create fake data
use plotters::prelude::*;
use rand::prelude::*;

pub fn random_numbers(n: i32, a: f64) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let mut v: Vec<f64> = Vec::new();
    for _ in 0..n {
        v.push(rng.gen_range(-a..a));
    }
    v
}

// function to generate fake data
pub fn data(len: i32, range: f64, noisemax: f64) -> (Vec<f64>, Vec<f64>) {
    let x: Vec<f64> = random_numbers(len, range);
    let noise: Vec<f64> = random_numbers(len, noisemax);
    let y: Vec<f64> = x
        .iter()
        .zip(noise.iter())
        .map(|(x, noise)| x.powi(2) + noise)
        .collect();
    (x, y)
}

// function to calculate the correlation between two vectors
pub fn correlation(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
    let x_mean = x.iter().sum::<f64>() / x.len() as f64;
    let y_mean = y.iter().sum::<f64>() / y.len() as f64;
    let x_std = x.iter().map(|x| (x - x_mean).powi(2)).sum::<f64>().sqrt();
    let y_std = y.iter().map(|y| (y - y_mean).powi(2)).sum::<f64>().sqrt();
    let xy = x
        .iter()
        .zip(y.iter())
        .map(|(x, y)| (x - x_mean) * (y - y_mean))
        .sum::<f64>();
    xy / (x_std * y_std)
}

pub fn plot(x: Vec<f64>, y: Vec<f64>, range: f64, noisemax: f64) {
    let data: Vec<(f64, f64)> = x.iter().cloned().zip(y.iter().cloned()).collect();
    let full_range = range.powi(2) + noisemax;
    let root_area = BitMapBackend::new("plot.png", (700, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 55.0)
        .set_label_area_size(LabelAreaPosition::Bottom, 55.0)
        .set_label_area_size(LabelAreaPosition::Right, 55.0)
        .set_label_area_size(LabelAreaPosition::Top, 55.0)
        .caption("Scatter Plot", ("sans-serif", 45.0))
        .build_cartesian_2d(-range..range, -0.0..full_range)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    // Draw Scatter Plot
    ctx.draw_series(data.iter().map(|point| Circle::new(*point, 4.0_f64, BLUE)))
        .unwrap();
    root_area.present().unwrap();
    println!("Plot finished");
}
