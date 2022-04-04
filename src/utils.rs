use crate::prelude::*;

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
                        (col + offset).min(self.bottom_right.0),
                        (row + offset).min(self.bottom_right.1),
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
}

pub fn draw_window_to_image(window: &Window, image: &mut SimilarityImage, val: f32) {
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
}
