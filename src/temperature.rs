
pub struct Temperature {
    celsius: f32,
    fahrenheit: f32
}

impl Temperature {
    pub fn from_celsius(value: f32) -> Self {
        Temperature {
            celsius: value,
            fahrenheit: value * 1.8 + 32.0
        }
    }

    pub fn from_fahrenheit(value: f32) -> Self {
        Temperature {
            celsius: (value - 32.0) / 1.8,
            fahrenheit: value
        }
    }

    pub fn get_celsius(&self) -> f32 { self.celsius }
    pub fn get_fahrenheit(&self) -> f32 { self.fahrenheit }
}