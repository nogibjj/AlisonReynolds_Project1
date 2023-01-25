// Function to create fake data

pub fn data(noise: &f64) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let x: Vec<f64> = (0..100).map(|x| x as f64).collect();
    let y: Vec<f64> = (0..100).map(|x| (x as f64) - noise).collect();
    let z: Vec<f64> = x.iter().zip(y.iter()).map(|(x, y)| x.powi(2) + y).collect();
    (x, y, z)
}
