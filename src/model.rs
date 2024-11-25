pub struct PostMeasurement {
    id: u32,
    measurement: Measurement
}


pub struct Measurement {
    pub blood_pressure: u32,
    pub heart_rate: u32,
    pub body_temperature: f32,
}