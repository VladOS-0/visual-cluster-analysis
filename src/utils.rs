pub fn rand_f32_in_range(min: f32, max: f32) -> f32 {
    assert!(max >= min, "Максимальное число больше минимального");
    let random_num: f32 = rand::random();
    (random_num * (max - min) + min).round()
}
