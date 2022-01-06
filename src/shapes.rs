use crate::constants::SCENE_SIZE;
use anyhow::{anyhow, Result};

/// Important to note that this is a point in universal, continous coordinates.
#[derive(Copy, Clone)]
pub struct Point {
    x: f32,
    y: f32,
}

fn check_ranges<N: PartialOrd + ToString>(values: Vec<N>, min: N, max: N) -> Result<()> {
    let mut wrong_vals = values.iter().filter(|v| **v < min || **v > max).peekable();
    if wrong_vals.peek().is_some() {
        Err(
            anyhow!("Values for {} type given outside the [{}, {}] range. The following were the erronous ranges:{}", std::any::type_name::<N>(), min.to_string(), max.to_string(), wrong_vals.fold(String::from(" "),|acc, v| acc + &v.to_string())),
        )
    } else {
        Ok(())
    }
}

impl Point {
    pub fn new(x: f32, y: f32) -> Result<Point> {
        check_ranges(vec![x, y], 0.0, SCENE_SIZE as f32)?;
        Ok(Point { x, y })
    }
}

#[derive(Copy, Clone)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

fn is_hex_format(hex: &str) -> bool {
    hex.starts_with('#') && hex.len() == 7 && hex[1..].chars().all(|d| d.is_digit(16))
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Result<Color> {
        check_ranges(vec![r, g, b], 0.0, 1.0)?;
        Ok(Color { r, g, b })
    }

    pub fn from_hex(hex: &str) -> Result<Color> {
        Err(anyhow!(
            "Esta función está incompleta y no se debe llamar: 'from_hex()'"
        ))
    }
}

/// Note that a 'Line' isn't a straight 2-point line. It's composed of an arbitrary amount of
/// Points. It can represent the entire border encapsulating a polygon. If a line circles back then
/// the last Point will be equal to the first one.
pub type Line = Vec<Point>;

pub struct Polygon {
    /// The borders being a Vec<Line> doesn't mean that every straight line encapsulating for
    /// example a square is a different border. That would be a polygon considered having just one border. The multiple borders are for polygons that have "holes" in them, like hollowed out circles.
    borders: Vec<Line>,

    /// Border color being "None" just means to not draw an outline when in "color" and "texture"
    /// modes.
    border_color: Option<Color>,

    /// If fill color is "None" it means the polygon shouldn't be filled in and only the lines
    /// should be drawn with Bresenham's.
    fill_color: Option<Color>,

    /// Layer to be drawn on.
    layer: i32,

    /// Id given in the svg.
    id: String,
}

impl Polygon {
    pub fn new(layer: i32, id: String) -> Polygon {
        Polygon {
            borders: Vec::new(),
            border_color: None,
            fill_color: None,
            layer,
            id,
        }
    }

    pub fn add_border(&mut self, border: Line) {
        self.borders.push(border);
    }

    pub fn get_borders(&self) -> &Vec<Line> {
        &self.borders
    }

    pub fn set_stroke_color(&mut self, color: Option<Color>) {
        self.border_color = color;
    }

    pub fn set_fill_color(&mut self, color: Option<Color>) {
        self.fill_color = color;
    }
}