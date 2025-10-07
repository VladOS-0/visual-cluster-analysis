pub fn rand_f32_in_range(min: f32, max: f32, decimal_places: i32) -> f32 {
    assert!(max >= min, "Максимальное число больше минимального");
    assert!(
        decimal_places >= 0,
        "Количество цифр после запятой не может быть отрицательным"
    );
    let random_num: f32 = rand::random();
    ((random_num * (max - min) + min) * 10.0_f32.powi(decimal_places)).round()
        / 10.0_f32.powi(decimal_places)
}
