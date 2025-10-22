pub fn rand_f32_in_range(min: f32, max: f32, decimal_places: u16) -> f32 {
    assert!(
        max >= min,
        "Максимальное число больше или равно минимальному"
    );

    let random_num: f32 = rand::random();
    (random_num * (max - min) + min).round_to_dp(decimal_places)
}

pub fn rand_f32_in_range_with_distance(
    min: f32,
    max: f32,
    distance: f32,
    decimal_places: u16,
) -> f32 {
    assert!(
        max >= min,
        "Максимальное число больше или равно минимальному"
    );
    assert!(distance >= 0.0, "Дистанция должна быть больше нуля");
    assert!(
        distance <= max - min,
        "Дистанция не должна превышать разность максимального и минимального числа"
    );

    let possible_values_count = ((max - min) / distance).floor() as isize;
    let random_num: f32 = rand_isize_in_range(0, possible_values_count) as f32 * distance;

    (random_num + min).round_to_dp(decimal_places)
}

pub fn rand_isize_in_range(min: isize, max: isize) -> isize {
    assert!(
        max >= min,
        "Максимальное число больше или равно минимальному"
    );

    rand_f32_in_range(min as f32, max as f32, 0) as isize
}

pub trait RandGet {
    type Element;

    fn rand_get<'a>(&'a self) -> Option<&'a Self::Element>;

    fn rand_get_mut<'a>(&'a mut self) -> Option<&'a mut Self::Element>;

    fn rand_index<'a>(&'a self) -> Option<usize>;
}

impl<T> RandGet for Vec<T> {
    type Element = T;

    fn rand_get<'a>(&'a self) -> Option<&'a Self::Element> {
        if self.is_empty() {
            return None;
        }
        self.get(rand_isize_in_range(0, (self.len() - 1) as isize) as usize)
    }

    fn rand_get_mut<'a>(&'a mut self) -> Option<&'a mut Self::Element> {
        if self.is_empty() {
            return None;
        }
        let self_len = self.len();
        self.get_mut(rand_isize_in_range(0, (self_len - 1) as isize) as usize)
    }

    fn rand_index<'a>(&'a self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }
        Some(rand_isize_in_range(0, (self.len() - 1) as isize) as usize)
    }
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
