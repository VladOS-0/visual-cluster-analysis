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

pub trait Cache<Element: PartialEq> {
    fn find_in_cache<'a>(&'a self, sought: Element) -> Option<&'a Element>;
}

impl<Element: PartialEq, T: AsRef<[Element]>> Cache<Element> for T {
    fn find_in_cache<'a>(&'a self, sought: Element) -> Option<&'a Element> {
        self.as_ref().iter().find(|element| **element == sought)
    }
}

pub trait CacheWithPredicate<Element> {
    fn predicate_find_in_cache<'a, P: FnMut(&&Element) -> bool>(
        &'a self,
        predicate: P,
    ) -> Option<&'a Element>;
}

impl<Element, T: AsRef<[Element]>> CacheWithPredicate<Element> for T {
    fn predicate_find_in_cache<'a, P: FnMut(&&Element) -> bool>(
        &'a self,
        predicate: P,
    ) -> Option<&'a Element> {
        self.as_ref().iter().find(predicate)
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
