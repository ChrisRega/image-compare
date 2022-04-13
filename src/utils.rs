use crate::prelude::*;
use image::GrayImage;

pub trait Decompose {
    fn split_channels(&self) -> [GrayImage; 3];
}

impl Decompose for RgbImage {
    fn split_channels(&self) -> [GrayImage; 3] {
        let mut red = GrayImage::new(self.width(), self.height());
        let mut green = red.clone();
        let mut blue = red.clone();
        Window::from_image(&red).iter_pixels().for_each(|p| {
            let data = self.get_pixel(p.0, p.1);
            red.put_pixel(p.0, p.1, Luma([data[0]]));
            green.put_pixel(p.0, p.1, Luma([data[1]]));
            blue.put_pixel(p.0, p.1, Luma([data[2]]));
        });
        [red, green, blue]
    }
}

pub fn merge_similarity_channels(input: &[&GraySimilarityImage; 3]) -> RGBSimilarityImage {
    let mut output = RGBSimilarityImage::new(input[0].width(), input[0].height());
    Window::new((0, 0), (output.width() - 1, output.height() - 1))
        .iter_pixels()
        .for_each(|p| {
            output.put_pixel(
                p.0,
                p.1,
                Rgb([
                    input[0].get_pixel(p.0, p.1)[0],
                    input[1].get_pixel(p.0, p.1)[0],
                    input[2].get_pixel(p.0, p.1)[0],
                ]),
            )
        });
    output
}

pub struct Window {
    pub top_left: (u32, u32),
    pub bottom_right: (u32, u32),
}

pub struct WindowIter<'a> {
    current_index: u32,
    window: &'a Window,
}

impl<'a> Iterator for WindowIter<'a> {
    type Item = (u32, u32);
    fn next(&mut self) -> Option<Self::Item> {
        let result = Some((
            self.window.top_left.0 + (self.current_index % self.window.width()),
            self.window.top_left.1 + (self.current_index / self.window.width()),
        ));
        self.current_index += 1;
        if self.current_index <= self.window.area() {
            result
        } else {
            None
        }
    }
}

impl Window {
    pub fn new(top_left: (u32, u32), bottom_right: (u32, u32)) -> Window {
        Window {
            top_left,
            bottom_right,
        }
    }
    pub fn width(&self) -> u32 {
        self.bottom_right.0 - self.top_left.0 + 1
    }

    pub fn height(&self) -> u32 {
        self.bottom_right.1 - self.top_left.1 + 1
    }

    pub fn area(&self) -> u32 {
        self.width() * self.height()
    }

    pub fn subdivide_by_offset(&self, offset: u32) -> Vec<Window> {
        let mut result = Vec::new();
        for col in (self.top_left.0..self.width()).step_by(offset as usize) {
            for row in (self.top_left.1..self.height()).step_by(offset as usize) {
                result.push(Window::new(
                    (col, row),
                    (
                        (col + offset - 1).min(self.bottom_right.0),
                        (row + offset - 1).min(self.bottom_right.1),
                    ),
                ))
            }
        }
        result
    }

    pub fn iter_pixels(&self) -> WindowIter {
        WindowIter {
            window: self,
            current_index: 0,
        }
    }

    pub fn from_image(image: &GrayImage) -> Window {
        Window {
            top_left: (0, 0),
            bottom_right: (image.width() - 1, image.height() - 1),
        }
    }
}

pub fn draw_window_to_image(window: &Window, image: &mut GraySimilarityImage, val: f32) {
    window
        .iter_pixels()
        .for_each(|current_pixel| image.put_pixel(current_pixel.0, current_pixel.1, Luma([val])));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_test() {
        let rows = 2;
        let cols = 9;
        let window = Window::new((1, 1), (cols, rows));
        assert_eq!(window.height(), rows);
        assert_eq!(window.width(), cols);
        assert_eq!(window.area(), rows * cols);
    }

    #[test]
    fn window_test_edge() {
        let window = Window::new((0, 0), (0, 0));
        assert_eq!(window.height(), 1);
        assert_eq!(window.width(), 1);
        assert_eq!(window.area(), 1);
    }

    #[test]
    fn window_iterator_test() {
        let window = Window::new((0, 0), (3, 2));
        let mut iter = window.iter_pixels();

        let next = iter.next().expect("iterator should work");
        assert_eq!(next.0, 0);
        assert_eq!(next.1, 0);

        //expect column-first iteration
        let next = iter.next().expect("iterator should work");
        assert_eq!(next.0, 1);
        assert_eq!(next.1, 0);

        //row break works
        let next = iter.nth(3).expect("iterator should work");
        assert_eq!(next.0, 1);
        assert_eq!(next.1, 1);
    }

    #[test]
    fn window_subdivide_test() {
        let window = Window::new((0, 0), (8, 7));
        let windows = window.subdivide_by_offset(8);
        //all windows areas combined are the original area
        assert_eq!(windows.iter().map(|w| w.area()).sum::<u32>(), window.area());
    }

    #[test]
    fn from_image_test() {
        let img = GrayImage::new(127, 244);
        let window = Window::from_image(&img);
        assert_eq!(window.bottom_right.0, 126);
        assert_eq!(window.bottom_right.1, 243);
    }
}
