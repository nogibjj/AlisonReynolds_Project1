// Function to create fake data
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
