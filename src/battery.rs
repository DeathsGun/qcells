#[derive(Debug)]
pub struct Battery {
    pub capacity: f32,
    pub temperature_high: f32,
    pub temperature_low: f32,
    pub voltage: f64,
}