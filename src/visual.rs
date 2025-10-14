use std::{
    collections::HashMap,
    ops::Sub,
    path::Path,
    process::{Command, exit},
};

use image::{
    Pixel, Rgba, RgbaImage,
    imageops::{FilterType, resize},
};

use crate::{
    font::CharSymbol,
    geometry::{Axis, FixedPosition, Point, Rectangle},
    utils::rand_f32_in_range,
};

const MAX_IMAGE_DIMENSION: u32 = 5_000;

const IMAGE_PADDING: u32 = 1;

const POINT_ALPHA: f32 = 1.0;

const FILLING_COLOR: &str = "#FFFFFF";
const FILLING_ALPHA: f32 = 1.0;

const GRAPH_X_STEP: f32 = 0.1;

/// Class ID - (Core Color, Point Color)
type ClassColors = HashMap<usize, (Color, Color)>;

#[derive(Clone)]
pub struct Image<T: AsRef<Path>> {
    path: T,
    inner: RgbaImage,
    rect: Rectangle,
    class_colors: ClassColors,

    final_width: u32,
    final_height: u32,
}

impl<T: AsRef<Path>> Image<T> {
    pub fn new(
        path: T,
        rect: Rectangle,
        fill: bool,
        custom_width: Option<u32>,
        custom_height: Option<u32>,
    ) -> Self {
        let mut width: u32 = rect.width().ceil() as u32 + IMAGE_PADDING;
        let mut height: u32 = rect.height().ceil() as u32 + IMAGE_PADDING;

        let mut final_width = width;
        let mut final_height = height;

        if let Some(custom_width) = custom_width
            && custom_width != 0
        {
            final_width = custom_width;
        }

        if let Some(custom_height) = custom_height
            && custom_height != 0
        {
            final_height = custom_height;
        }

        if width > MAX_IMAGE_DIMENSION || height > MAX_IMAGE_DIMENSION {
            println!(
                "ВНИМАНИЕ: максимальные высота и ширина изображения равны {}, однако требуется создать холст {}/{}. Изображение будет сжато, что может привести к потере некоторых точек.",
                MAX_IMAGE_DIMENSION, height, width
            );

            let downscale_ratio: f32;
            if width > height {
                downscale_ratio = width as f32 / MAX_IMAGE_DIMENSION as f32
            } else {
                downscale_ratio = height as f32 / MAX_IMAGE_DIMENSION as f32
            }
            width = (width as f32 / downscale_ratio).round() as u32;
            height = (height as f32 / downscale_ratio).round() as u32;

            println!(
                "Изображение было сжато до {}/{} (в {} раз)",
                height, width, downscale_ratio
            );
        }

        if final_width > MAX_IMAGE_DIMENSION || final_height > MAX_IMAGE_DIMENSION {
            println!(
                "ВНИМАНИЕ: максимальные КАСТОМНЫЕ высота и ширина изображения равны {}, однако требуется создать холст {}/{}. Изображение будет сжато.",
                MAX_IMAGE_DIMENSION, final_height, final_width
            );
            let downscale_ratio: f32;

            if final_width > final_height {
                downscale_ratio = final_width as f32 / MAX_IMAGE_DIMENSION as f32
            } else {
                downscale_ratio = final_height as f32 / MAX_IMAGE_DIMENSION as f32
            }
            final_width = (final_width as f32 / downscale_ratio).round() as u32;
            final_height = (final_height as f32 / downscale_ratio).round() as u32;

            println!(
                "Изображение (КАСТОМНОЕ) было сжато до {}/{} (в {} раз)",
                final_height, final_width, downscale_ratio
            );
        }

        let mut inner = RgbaImage::new(width, height);
        if fill {
            for pixel in inner.pixels_mut() {
                *pixel = Color::hex(FILLING_COLOR, FILLING_ALPHA).inner();
            }
        }

        let class_colors = Self::init_default_colors();

        Self {
            path,
            inner,
            rect,
            class_colors,
            final_width,
            final_height,
        }
    }

    fn init_default_colors() -> ClassColors {
        let mut class_colors = HashMap::new();

        class_colors.insert(
            1,
            (
                Color::hex("#1e6b8a", POINT_ALPHA),
                Color::hex("#4a9fc8", POINT_ALPHA),
            ),
        );

        class_colors.insert(
            2,
            (
                Color::hex("#8a351e", POINT_ALPHA),
                Color::hex("#c86a4a", POINT_ALPHA),
            ),
        );

        class_colors.insert(
            3,
            (
                Color::hex("#2d8a1e", POINT_ALPHA),
                Color::hex("#52c84a", POINT_ALPHA),
            ),
        );

        class_colors.insert(
            4,
            (
                Color::hex("#7a1e8a", POINT_ALPHA),
                Color::hex("#b84ac8", POINT_ALPHA),
            ),
        );

        class_colors.insert(
            5,
            (
                Color::hex("#8a7a1e", POINT_ALPHA),
                Color::hex("#c8b84a", POINT_ALPHA),
            ),
        );

        class_colors.insert(
            6,
            (
                Color::hex("#1e8a7a", POINT_ALPHA),
                Color::hex("#4ac8b8", POINT_ALPHA),
            ),
        );

        class_colors.insert(
            7,
            (
                Color::hex("#8a2d1e", POINT_ALPHA),
                Color::hex("#c85a4a", POINT_ALPHA),
            ),
        );

        class_colors.insert(
            8,
            (
                Color::hex("#4a1e8a", POINT_ALPHA),
                Color::hex("#7a4ac8", POINT_ALPHA),
            ),
        );

        class_colors.insert(
            9,
            (
                Color::hex("#8a1e4a", POINT_ALPHA),
                Color::hex("#c84a7a", POINT_ALPHA),
            ),
        );

        class_colors.insert(
            10,
            (
                Color::hex("#5a8a1e", POINT_ALPHA),
                Color::hex("#8ac84a", POINT_ALPHA),
            ),
        );

        class_colors.insert(
            11,
            (
                Color::hex("#1e5a8a", POINT_ALPHA),
                Color::hex("#4a8ac8", POINT_ALPHA),
            ),
        );

        class_colors.insert(
            12,
            (
                Color::hex("#8a5a1e", POINT_ALPHA),
                Color::hex("#c88a4a", POINT_ALPHA),
            ),
        );

        class_colors.insert(
            13,
            (
                Color::hex("#6a1e8a", POINT_ALPHA),
                Color::hex("#9a4ac8", POINT_ALPHA),
            ),
        );

        class_colors.insert(
            14,
            (
                Color::hex("#1e8a2d", POINT_ALPHA),
                Color::hex("#4ac85a", POINT_ALPHA),
            ),
        );

        class_colors.insert(
            15,
            (
                Color::hex("#8a6a1e", POINT_ALPHA),
                Color::hex("#c89a4a", POINT_ALPHA),
            ),
        );
        class_colors
    }

    pub fn draw_point_with_class(
        &mut self,
        point: Point,
        class: usize,
        is_core: bool,
        silent: bool,
    ) {
        let color = self
            .class_colors
            .entry(class)
            .or_insert_with(|| {
                let rand_point_color = Color::rand();
                let mut rand_core_color = rand_point_color.clone();
                rand_core_color.make_core();
                (rand_point_color, rand_core_color)
            })
            .clone();

        if is_core {
            self.draw_point_with_color(point, color.0, false, silent);
        } else {
            self.draw_point_with_color(point, color.1, true, silent);
        }
    }

    pub fn draw_point_with_color(
        &mut self,
        point: Point,
        color: Color,
        do_not_override: bool,
        silent: bool,
    ) {
        let x = point.x.floor() - self.rect.bottom_left.x;
        let y = self.rect.top_right.y - point.y.floor();

        let width_ratio = (self.inner.width() - IMAGE_PADDING) as f32 / self.rect.width();
        let height_ratio = (self.inner.height() - IMAGE_PADDING) as f32 / self.rect.height();

        let pixel = self.inner.get_pixel_mut_checked(
            (x * width_ratio).floor() as u32 + IMAGE_PADDING / 2,
            (y * height_ratio).floor() as u32 + IMAGE_PADDING / 2,
        );
        if let None = pixel {
            if !silent {
                eprintln!(
                    "ПРЕДУПРЕЖДЕНИЕ: не удалось отрисовать пиксель для точки {} по коориданатам ({}; {}); Поле - {}; Ширина изображения - {}, Высота изображения - {}",
                    point,
                    (x * width_ratio).floor() as u32 + IMAGE_PADDING / 2,
                    (y * height_ratio).floor() as u32 + IMAGE_PADDING / 2,
                    self.rect,
                    self.inner.width(),
                    self.inner.height()
                );
            }
            return;
        }
        let pixel = pixel.unwrap();
        if do_not_override && pixel.0 != Color::hex(FILLING_COLOR, FILLING_ALPHA).inner.0 {
            if !silent {
                println!(
                    "ПРЕДУПРЕЖДЕНИЕ: пиксель {} по коориданатам ({}; {}) накладывается на другой и отрисован не будет.",
                    point,
                    (x * width_ratio).floor() as u32 + IMAGE_PADDING / 2,
                    (x * width_ratio).floor() as u32 + IMAGE_PADDING / 2,
                );
            }
            return;
        }
        *pixel = color.inner();
    }

    pub fn draw_graph<K>(&mut self, func: &K, color: Option<Color>)
    where
        K: Fn(f32) -> Option<f32>,
    {
        let color = color.unwrap_or_else(|| Color::hex("#b90000", 0.6));

        let mut x = self.rect.bottom_left.x;

        while x <= self.rect.top_right.x {
            let y = func(x);
            if let Some(y) = y {
                self.draw_point_with_color(Point::new(x, y), color, false, true);
            }
            x += GRAPH_X_STEP;
        }
    }

    pub fn draw_axis(&mut self, axis: Axis, symbol: Option<CharSymbol>, color: Option<Color>) {
        let color = color.unwrap_or_else(|| Color::hex("#000000", 0.3));

        match axis {
            Axis::X => {
                let mut x = self.rect.bottom_left.x;
                while x <= self.rect.top_right.x {
                    self.draw_point_with_color(Point::new(x, 0.0), color, false, true);
                    x += 1.0;
                }

                self.draw_symbol(
                    self.rect
                        .get_position(FixedPosition::MiddleRight(-8.0, 2.0)),
                    color,
                    symbol.unwrap_or(CharSymbol::get('X')),
                )
            }
            Axis::Y => {
                let mut y = self.rect.bottom_left.y;
                while y <= self.rect.top_right.y {
                    self.draw_point_with_color(Point::new(0.0, y), color, false, true);
                    y += 1.0;
                }

                self.draw_symbol(
                    self.rect.get_position(FixedPosition::TopMiddle(2.0, -6.0)),
                    color,
                    symbol.unwrap_or(CharSymbol::get('Y')),
                );
            }
            Axis::Other(func) => {
                let symbol_point_y = func(self.rect.top_right.x - 5.0)
                    .unwrap_or(0.0)
                    .clamp(self.rect.bottom_left.y, self.rect.top_right.y)
                    .sub(9.0)
                    .clamp(self.rect.bottom_left.y, self.rect.top_right.y);
                let symbol_point = Point::new(self.rect.top_right.x - 9.0, symbol_point_y);

                self.draw_symbol(symbol_point, color, symbol.unwrap_or(CharSymbol::get('Z')));

                self.draw_graph(&func, Some(color));
            }
        }
    }

    pub fn write<K: AsRef<str>>(&mut self, bottom_left: Point, text: K, color: Option<Color>) {
        let mut point = bottom_left.clone();
        let color = color.unwrap_or_else(|| Color::hex("#000000", 1.0));
        for char in text.as_ref().chars() {
            let symbol = CharSymbol::get(char);
            self.draw_symbol(point, color, symbol);
            point.x += (symbol.width() + 1) as f32;
        }
    }

    pub fn draw_symbol(&mut self, bottom_left: Point, color: Color, symbol: CharSymbol) {
        let mut point = bottom_left.clone();
        for line in symbol.0.lines().rev() {
            point.x = bottom_left.x;
            let mut touched_line = false;
            for char in line.chars() {
                if char != '\n' {
                    touched_line = true;
                }
                if char != ' ' && char != '\n' {
                    self.draw_point_with_color(point, color, false, true);
                }
                point.x += 1.0;
            }
            if touched_line {
                point.y += 1.0;
            }
        }
    }

    pub fn save(&mut self) {
        let resized_image;
        if self.final_height != self.inner.height() || self.final_width != self.inner.width() {
            resized_image = resize(
                &self.inner,
                self.final_width,
                self.final_height,
                FilterType::Nearest,
            );

            let result = resized_image.save(self.path.as_ref());
            if let Err(err) = result {
                eprintln!(
                    "ОШИБКА: Не удалось сохранить изображение по пути {}: {}",
                    self.path.as_ref().to_string_lossy(),
                    err
                );
            }
        } else {
            let result = self.inner.save(self.path.as_ref());
            if let Err(err) = result {
                eprintln!(
                    "ОШИБКА: Не удалось сохранить изображение по пути {}: {}",
                    self.path.as_ref().to_string_lossy(),
                    err
                );
            }
        }

        println!(
            "Изображение сохранено по пути {}",
            self.path.as_ref().to_string_lossy()
        );
    }

    pub fn show(&self, command: &str) {
        let result = Command::new(command).arg(self.path.as_ref()).spawn();

        if let Err(err) = result {
            eprintln!(
                "ОШИБКА: Не удалось открыть изображение по пути {} с помощью команды {}: {}",
                self.path.as_ref().to_string_lossy(),
                command,
                err
            );
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    inner: Rgba<u8>,
}

impl Color {
    pub fn hex(hex: &str, alpha: f32) -> Self {
        if !hex.starts_with("#") {
            eprintln!("ОШИБКА: Цвет {} не начинается с #!", hex);
            exit(1);
        }
        if alpha < 0.0 || alpha > 1.0 {
            eprintln!("ОШИБКА: Альфа-канал {} не лежит в пределах (0; 1)!", alpha);
            exit(1);
        }
        let hex = u32::from_str_radix(&hex[1..hex.len()], 16).unwrap_or_else(|err| {
            eprintln!(
                "ОШИБКА: Не удалось распарсить {} как шестнадцатеричный цвет: {}",
                hex, err
            );
            exit(1);
        });

        let r = ((hex & 0xff0000) >> 16) as u8;
        let g = ((hex & 0xff00) >> 8) as u8;
        let b = (hex & 0xff) as u8;
        let a = (alpha * 255.0) as u8;

        Self::rgba(r, g, b, a)
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        let inner: Rgba<u8> = Rgba::from_slice(&[r, g, b, a]).to_owned();
        Self { inner }
    }

    pub fn make_core(&mut self) {
        if self.inner.0[0] >= 30 {
            self.inner.0[0] -= 30;
        } else {
            self.inner.0[0] = rand_f32_in_range(0.0, self.inner.0[0] as f32, 0) as u8
        }

        if self.inner.0[1] >= 30 {
            self.inner.0[1] -= 30;
        } else {
            self.inner.0[1] = rand_f32_in_range(0.0, self.inner.0[2] as f32, 0) as u8
        }

        if self.inner.0[2] >= 30 {
            self.inner.0[2] -= 30;
        } else {
            self.inner.0[2] = rand_f32_in_range(0.0, self.inner.0[2] as f32, 0) as u8
        }
    }

    pub fn rand() -> Self {
        let r = rand_f32_in_range(0.0, 255.0, 0) as u8;
        let g = rand_f32_in_range(0.0, 255.0, 0) as u8;
        let b = rand_f32_in_range(0.0, 255.0, 0) as u8;
        Self::rgba(r, g, b, (POINT_ALPHA * 255.0) as u8)
    }

    pub fn inner(self) -> Rgba<u8> {
        self.inner
    }
}
