use core::f32;

use crate::utils::{Cache, RoundToDecimalPlaces};

#[derive(Default)]
pub struct Random<'a> {
    min: Option<f32>,
    max: Option<f32>,
    bias_range_min: Option<f32>,
    bias_range_max: Option<f32>,
    bias_mult: Option<f32>,
    bias_prob: Option<f32>,
    cache_regenerate_attempts: Option<usize>,
    cache: Option<Box<&'a dyn Cache<f32>>>,
    decimal_places: Option<u16>,
    distance: Option<f32>,
}

impl<'a> Random<'a> {
    pub fn prob(probability: f32) -> bool {
        debug_assert!(
            probability >= 0.0 && probability <= 1.0,
            "Вероятность ({}) должно лежать в пределах 0.0 - 1.0",
            probability
        );
        probability <= Self::new().generate()
    }

    pub fn new() -> Self {
        Self::default()
    }
    pub fn range(self, min: f32, max: f32) -> Self {
        debug_assert!(max >= min, "Максимальное число больше минимального");

        debug_assert!(
            if self.bias_range_min.is_some() {
                *self.bias_range_min.as_ref().unwrap() >= min
                    && *self.bias_range_max.as_ref().unwrap() <= max
            } else {
                true
            },
            "bias_range_min ({}) - bias_range_max ({}) должны лежать в пределах min ({}) - max ({})",
            self.bias_range_min.unwrap(),
            self.bias_range_max.unwrap(),
            min,
            max,
        );

        debug_assert!(
            self.distance.is_none() || *self.distance.as_ref().unwrap() <= max - min,
            "Дистанция не должна превышать разность максимального и минимального числа"
        );

        Self {
            min: Some(min),
            max: Some(max),
            ..self
        }
    }

    pub fn bias(
        self,
        bias_mult: f32,
        bias_prob: f32,
        bias_range_min: Option<f32>,
        bias_range_max: Option<f32>,
    ) -> Self {
        debug_assert!(
            (bias_range_min.is_some() && bias_range_max.is_some())
                || (bias_range_min.is_none() && bias_range_max.is_none()),
            "bias_range_min ({:?}) и bias_range_max ({:?}) должны быть либо одновременно указаны, либо одновременно не указаны",
            bias_range_min,
            bias_range_max
        );
        debug_assert!(
            if bias_range_min.is_some() {
                bias_range_min.as_ref().unwrap() <= bias_range_max.as_ref().unwrap()
            } else {
                true
            },
            "bias_range_min ({}) больше bias_range_max ({})",
            bias_range_min.unwrap(),
            bias_range_max.unwrap(),
        );

        debug_assert!(
            if bias_range_min.is_some() && self.min.is_some() {
                bias_range_min.as_ref().unwrap() >= self.min.as_ref().unwrap()
                    && bias_range_max.as_ref().unwrap() <= self.max.as_ref().unwrap()
            } else {
                true
            },
            "bias_range_min ({}) - bias_range_max ({}) должны лежать в пределах min ({}) - max ({})",
            bias_range_min.unwrap(),
            bias_range_max.unwrap(),
            self.min.as_ref().unwrap(),
            self.max.as_ref().unwrap(),
        );

        debug_assert!(
            bias_prob >= 0.0 && bias_prob <= 1.0,
            "bias_prob () должно лежать в пределах 0.0 - 1.0"
        );

        Self {
            bias_mult: Some(bias_mult),
            bias_prob: Some(bias_prob),
            bias_range_min,
            bias_range_max,
            ..self
        }
    }

    pub fn cache<T: Cache<f32>>(self, cache: &'a T, cache_regenerate_attempts: usize) -> Self {
        debug_assert!(
            cache_regenerate_attempts > 0,
            "cache_regenerate_attempts ({}) должно быть больше нуля",
            cache_regenerate_attempts
        );

        Self {
            cache: Some(Box::new(cache)),
            cache_regenerate_attempts: Some(cache_regenerate_attempts),
            ..self
        }
    }

    pub fn distance(self, distance: f32) -> Self {
        debug_assert!(distance >= 0.0, "Дистанция должна быть больше нуля");
        debug_assert!(
            self.min.is_none()
                || distance <= self.max.as_ref().unwrap() - self.min.as_ref().unwrap(),
            "Дистанция не должна превышать разность максимального и минимального числа"
        );
        Self {
            distance: Some(distance),
            ..self
        }
    }

    pub fn to_dp(self, decimal_places: u16) -> Self {
        Self {
            decimal_places: Some(decimal_places),
            ..self
        }
    }

    pub fn generate(&self) -> f32 {
        let mut result = 0.0;
        for _ in 0..self.cache_regenerate_attempts.unwrap_or(1) {
            result = self.attempt_generate();
            if self
                .cache
                .as_ref()
                .is_none_or(|cache| cache.find_in_cache(result).is_none())
            {
                break;
            }
        }
        result
    }

    fn attempt_generate(&self) -> f32 {
        let mut result: f32 = if let (Some(min), Some(max)) = (self.min, self.max) {
            rand::random_range(min..=max)
        } else {
            rand::random()
        };

        if let (Some(bias_range_min), Some(bias_range_max)) =
            (self.bias_range_min, self.bias_range_max)
        {
            if result >= bias_range_min
                && result <= bias_range_max
                && Random::prob(self.bias_prob.unwrap())
            {
                result *= self.bias_mult.unwrap();
            }
        } else if let Some(bias_prob) = self.bias_prob
            && Random::prob(bias_prob)
        {
            result *= self.bias_mult.unwrap();
        }

        if let Some(distance) = self.distance {
            let remainder = result % distance;

            result -= remainder;
        }

        if let Some(dp) = self.decimal_places {
            result = result.round_to_dp(dp);
        }

        if let (Some(min), Some(max)) = (self.min, self.max) {
            result = result.clamp(min, max);
        }

        result
    }
}

/*
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
*/
