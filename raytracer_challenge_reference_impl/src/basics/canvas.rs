use std::fmt;

use serde_derive::{Deserialize, Serialize};

use crate::basics::{Color, Pixel};

pub type ColorVec = Vec<Color>;
pub type PixelVec = Vec<Pixel>;

#[derive(Clone, Debug, PartialEq)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixel: PixelVec,
}

pub trait CanvasOps {
    fn new(width: usize, height: usize) -> Self;
    fn empty() -> Canvas;
    fn write_pixel(&mut self, x: usize, y: usize, c: Color);
    fn pixel_at(&self, x: usize, y: usize) -> &Pixel;

    fn calc_idx(&self, x: usize, y: usize) -> usize;

    fn get_pixels_mut(&mut self) -> &mut PixelVec;
    fn get_pixels(&self) -> &PixelVec;

    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;

    fn set_width(&mut self, w: usize);
    fn set_height(&mut self, h: usize);
}

impl CanvasOps for Canvas {
    fn new(width: usize, height: usize) -> Self {
        let mut c = Canvas {
            width,
            height,
            pixel: vec![Pixel::new(); width * height],
        };
        for x in 0..width {
            for y in 0..height {
                c.pixel[y * width + x].x = x;
                c.pixel[y * width + x].y = y;
            }
        }

        c
    }

    fn empty() -> Canvas {
        Canvas {
            width: 0,
            height: 0,
            pixel: vec![],
        }
    }

    fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        assert!(x < self.width);
        assert!(y < self.height);
        //println!("write_pixel   x {}  y {}   c   {:?} ", x, y, &c);
        self.pixel[y * self.width + x].color = c;
    }

    fn pixel_at(&self, x: usize, y: usize) -> &Pixel {
        assert!(x < self.width);
        assert!(y < self.height);

        &self.pixel[self.calc_idx(x, y)]
    }

    fn calc_idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get_pixels_mut(&mut self) -> &mut Vec<Pixel> {
        &mut self.pixel
    }

    fn get_pixels(&self) -> &PixelVec {
        &self.pixel
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn set_width(&mut self, w: usize) {
        self.width = w;
    }

    fn set_height(&mut self, h: usize) {
        self.height = h;
    }
}

impl Canvas {
    pub fn tiles(&self, x_tiles: usize, y_tiles: usize) -> CanvasTile {
        let c = CanvasTile {
            x_inc: self.width / x_tiles,
            y_inc: self.height / y_tiles,
            width: self.width,
            height: self.height,
            x: 0,
            y: 0,
            idx: 0,
        };
        println!("canvas tile  {:?}", &c);
        c
    }
}

#[derive(PartialEq)]
pub struct Tile {
    x_from: usize,
    x_to: usize,
    y_from: usize,
    y_to: usize,
    idx: usize,
}

impl Tile {
    pub fn x_from(&self) -> usize {
        self.x_from
    }

    pub fn x_to(&self) -> usize {
        self.x_to
    }

    pub fn y_from(&self) -> usize {
        self.y_from
    }

    pub fn y_to(&self) -> usize {
        self.y_to
    }

    pub fn get_idx(&self) -> usize {
        self.idx
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CanvasTile {
    x: usize,
    y: usize,
    x_inc: usize,
    y_inc: usize,
    width: usize,
    height: usize,
    idx: usize,
}

impl Iterator for CanvasTile {
    type Item = Tile;

    fn next(&mut self) -> Option<Tile> {
        if self.y >= self.height {
            None
        } else {
            let x_from = self.x;
            let x_to = if self.x + self.x_inc > self.width {
                self.width
            } else {
                self.x + self.x_inc
            };

            let y_from = self.y;
            let y_to = if y_from + self.y_inc > self.height {
                self.height
            } else {
                y_from + self.y_inc
            };
            let tile = Tile {
                x_from,
                x_to,
                y_from,
                y_to,
                idx: self.idx,
            };

            self.idx += 1;
            self.x += self.x_inc;
            if self.x >= self.width {
                self.x = 0;
                self.y += self.y_inc;
            }

            Some(tile)
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct TileData {
    idx: usize,
    points: Vec<TileDataPoint>,
}

#[derive(Deserialize, Serialize)]
pub struct TileDataPoint {
    x: usize,
    y: usize,
    c: Color,
}

impl TileDataPoint {
    pub fn new(x: usize, y: usize, c: Color) -> TileDataPoint {
        TileDataPoint { x, y, c }
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }

    pub fn get_color(&self) -> Color {
        self.c
    }
}

impl TileData {
    pub fn new(idx: usize, points: Vec<TileDataPoint>) -> TileData {
        TileData { idx, points }
    }

    pub fn get_idx(&self) -> usize {
        self.idx
    }

    pub fn get_points(&self) -> &Vec<TileDataPoint> {
        &self.points
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "idx {}  {}/{} -> {}/{}",
            self.idx, self.x_from, self.y_from, self.x_to, self.y_to
        )
    }
}

impl fmt::Debug for TileDataPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}/{} -> ({}/{}/{})", self.x, self.y, self.c.r, self.c.g, self.c.b)
    }
}

impl fmt::Debug for TileData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "idx {} -> points.len {} ", self.idx, self.get_points().len())
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "idx {}    {}/{} -> {}/{}",
            self.idx, self.x_from, self.y_from, self.x_to, self.y_to
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::basics::{Canvas, CanvasOps, Tile};

    #[test]
    fn test_iterator1() {
        let c = Canvas::new(9, 9);

        let mut it = c.tiles(2, 2);

        let t1 = Tile {
            x_from: 0,
            y_from: 0,
            x_to: 4,
            y_to: 4,
            idx: 0,
        };
        let t2 = Tile {
            x_from: 4,
            y_from: 0,
            x_to: 8,
            y_to: 4,
            idx: 1,
        };
        let t3 = Tile {
            x_from: 8,
            y_from: 0,
            x_to: 9,
            y_to: 4,
            idx: 2,
        };

        let t4 = Tile {
            x_from: 0,
            y_from: 4,
            x_to: 4,
            y_to: 8,
            idx: 3,
        };
        let t5 = Tile {
            x_from: 4,
            y_from: 4,
            x_to: 8,
            y_to: 8,
            idx: 4,
        };
        let t6 = Tile {
            x_from: 8,
            y_from: 4,
            x_to: 9,
            y_to: 8,
            idx: 5,
        };

        let t7 = Tile {
            x_from: 0,
            y_from: 8,
            x_to: 4,
            y_to: 9,
            idx: 6,
        };
        let t8 = Tile {
            x_from: 4,
            y_from: 8,
            x_to: 8,
            y_to: 9,
            idx: 7,
        };
        let t9 = Tile {
            x_from: 8,
            y_from: 8,
            x_to: 9,
            y_to: 9,
            idx: 8,
        };

        assert_eq!(it.next(), Some(t1));
        assert_eq!(it.next(), Some(t2));
        assert_eq!(it.next(), Some(t3));
        assert_eq!(it.next(), Some(t4));
        assert_eq!(it.next(), Some(t5));
        assert_eq!(it.next(), Some(t6));
        assert_eq!(it.next(), Some(t7));
        assert_eq!(it.next(), Some(t8));
        assert_eq!(it.next(), Some(t9));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_iterator2() {
        let c = Canvas::new(9, 9);

        let mut it = c.tiles(3, 3);

        let t1 = Tile {
            x_from: 0,
            y_from: 0,
            x_to: 3,
            y_to: 3,
            idx: 0,
        };
        let t2 = Tile {
            x_from: 3,
            y_from: 0,
            x_to: 6,
            y_to: 3,
            idx: 1,
        };
        let t3 = Tile {
            x_from: 6,
            y_from: 0,
            x_to: 9,
            y_to: 3,
            idx: 2,
        };

        let t4 = Tile {
            x_from: 0,
            y_from: 3,
            x_to: 3,
            y_to: 6,
            idx: 3,
        };
        let t5 = Tile {
            x_from: 3,
            y_from: 3,
            x_to: 6,
            y_to: 6,
            idx: 4,
        };
        let t6 = Tile {
            x_from: 6,
            y_from: 3,
            x_to: 9,
            y_to: 6,
            idx: 5,
        };

        let t7 = Tile {
            x_from: 0,
            y_from: 6,
            x_to: 3,
            y_to: 9,
            idx: 6,
        };
        let t8 = Tile {
            x_from: 3,
            y_from: 6,
            x_to: 6,
            y_to: 9,
            idx: 7,
        };
        let t9 = Tile {
            x_from: 6,
            y_from: 6,
            x_to: 9,
            y_to: 9,
            idx: 8,
        };

        assert_eq!(it.next(), Some(t1));
        assert_eq!(it.next(), Some(t2));
        assert_eq!(it.next(), Some(t3));
        assert_eq!(it.next(), Some(t4));
        assert_eq!(it.next(), Some(t5));
        assert_eq!(it.next(), Some(t6));
        assert_eq!(it.next(), Some(t7));
        assert_eq!(it.next(), Some(t8));
        assert_eq!(it.next(), Some(t9));
        assert_eq!(it.next(), None);
    }
}
