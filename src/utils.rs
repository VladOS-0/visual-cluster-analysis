pub fn rand_f32_in_range(min: f32, max: f32, decimal_places: u16) -> f32 {
    assert!(max >= min, "Максимальное число больше минимального");

    let random_num: f32 = rand::random();
    (random_num * (max - min) + min).round_to_dp(decimal_places)
}

pub trait RoundToDecimalPlaces {
    fn round_to_dp(self, decimal_places: u16) -> Self
    where
        Self: Sized;
}

impl RoundToDecimalPlaces for f32 {
    fn round_to_dp(self, decimal_places: u16) -> Self
    where
        Self: Sized,
    {
        (self * 10.0_f32.powi(decimal_places as i32)).round() / 10.0_f32.powi(decimal_places as i32)
    }
}
