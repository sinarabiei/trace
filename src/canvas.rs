use crate::prelude::*;
use std::fs::File;
use std::io::Write;
use std::ops::{Index, IndexMut};

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    array: Vec<Color>,
}

impl Canvas {
    /// Creates a new `Canvas`, every pixel is
    /// initialized to black, `color![0, 0, 0]`.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            array: vec![color![0, 0, 0]; width * height],
        }
    }

    /// Returns a PPM-formatted string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use trace::prelude::*;
    /// let mut canvas = Canvas::new(5, 3);
    /// canvas[(0, 0)] = color![1.5, 0, 0];
    /// canvas[(2, 1)] = color![0, 0.5, 0];
    /// canvas[(4, 2)] = color![-0.5, 0, 1];
    /// let ppm = format!(
    ///               "{}\n{}\n{}\n{}\n{}\n{}\n",
    ///               "P3",
    ///               "5 3",
    ///               "255",
    ///               "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
    ///               "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0",
    ///               "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"
    ///           );
    /// assert_eq!(canvas.to_ppm(), ppm);
    ///
    /// // No line in a PPM file should be more than 70 characters long.
    /// let mut canvas = Canvas::new(10, 2);
    /// for width in 0..canvas.width {
    ///     for height in 0..canvas.height {
    ///         canvas[(width, height)] = color![1, 0.8, 0.6];
    ///     }
    /// }
    /// let ppm = format!(
    ///               "{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
    ///               "P3",
    ///               "10 2",
    ///               "255",
    ///               "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
    ///               "153 255 204 153 255 204 153 255 204 153 255 204 153",
    ///               "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
    ///               "153 255 204 153 255 204 153 255 204 153 255 204 153"
    ///           );
    /// assert_eq!(canvas.to_ppm(), ppm);
    /// ```
    pub fn to_ppm(&self) -> String {
        let mut ppm = String::new();
        ppm.push_str("P3\n");
        ppm.push_str(format!("{} {}\n", self.width, self.height).as_str());
        ppm.push_str("255\n");
        for height in 0..self.height {
            let mut char_count = 0;
            for width in 0..self.width {
                let pixel = self[(width, height)];
                let red = (pixel.red * 255.0).ceil().clamp(0.0, 255.0).to_string();
                let green = (pixel.green * 255.0).ceil().clamp(0.0, 255.0).to_string();
                let blue = (pixel.blue * 255.0).ceil().clamp(0.0, 255.0).to_string();
                char_count = push_color(&mut ppm, &red, char_count);
                char_count = push_color(&mut ppm, &green, char_count);
                char_count = push_color(&mut ppm, &blue, char_count);
            }
            ppm.push('\n');
        }
        ppm
    }

    /// Writes PPM-formatted string of canvas into `path`
    pub fn write(&self, path: &str) -> Result<(), std::io::Error> {
        File::create(path)?.write(self.to_ppm().as_bytes())?;
        Ok(())
    }
}

fn push_color(ppm: &mut String, color: &str, mut count: usize) -> usize {
    if count == 0 {
        ppm.push_str(color);
        count += color.len();
    } else if count + 1 + color.len() > 70 {
        ppm.push('\n');
        count = 0;
        ppm.push_str(color);
        count += color.len();
    } else {
        ppm.push(' ');
        ppm.push_str(color);
        count += 1 + color.len();
    }
    count
}

impl Index<(usize, usize)> for Canvas {
    type Output = Color;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        if index.0 < self.width && index.1 < self.height {
            return &self.array[index.1 * self.width + index.0];
        }
        panic!(
            "index out of bounds: canvas size is {} by {}, index is [({}, {})]",
            self.width, self.height, index.0, index.1
        );
    }
}

/// # Examples
///
/// ```
/// # use trace::prelude::*;
/// let mut canvas = Canvas::new(10, 20);
/// let red = color![1, 0, 0];
/// canvas[(2, 3)] = red;
/// assert_eq!(canvas[(2, 3)], red);
/// ```
impl IndexMut<(usize, usize)> for Canvas {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        if index.0 < self.width && index.1 < self.height {
            return &mut self.array[index.1 * self.width + index.0];
        }
        panic!(
            "index out of bounds: canvas size is {} by {}, index is [({}, {})]",
            self.width, self.height, index.0, index.1
        );
    }
}
