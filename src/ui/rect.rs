#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Rect {
    pub x                   : u32,
    pub y                   : u32,
    pub width               : u32,
    pub height              : u32,
}

impl Rect {

    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x               : x,
            y               : y,
            width           : width,
            height          : height
        }
    }

    pub fn new_with_size(width: u32, height: u32) -> Self {
        Self {
            x               : 0,
            y               : 0,
            width           : width,
            height          : height
        }
    }

    pub fn empty() -> Self {
        Self {
            x               : 0,
            y               : 0,
            width           : 0,
            height          : 0
        }
    }

    pub fn is_inside(&self, p: (u32, u32)) -> bool {
        p.0 >= self.x && p.1 >= self.y && p.0 < self.x + self.width && p.1 < self.y + self.height
    }

    pub fn to_usize(&self) -> (usize, usize, usize, usize) {
        (self.x as usize, self.y as usize, self.width as usize, self.height as usize)
    }
}