use maths_rs::Vec3f;

#[derive(PartialEq, Debug, Clone)]
pub struct Palette {

    pub colors                      : Vec<[u8; 4]>,
    pub colors_f                    : Vec<[f32; 4]>,
}

impl Palette {
    pub fn new() -> Self {
        Self {
            colors                  : vec![],
            colors_f                : vec![],
        }
    }

    /// Load the palette from a Paint.net TXT file
    pub fn load_from_txt(&mut self, txt: String) {
        for line in txt.lines() {

            // Ignore comments
            if line.starts_with(';') {
                continue;
            }

            let mut chars = line.chars();

            // Skip Alpha
            if chars.next().is_none() { return; }
            if chars.next().is_none() { return; }

            // R
            let mut r_string = "".to_string();
            if let Some(c) = chars.next() {
                r_string.push(c);
            }
            if let Some(c) = chars.next() {
                r_string.push(c);
            }

            let r = u8::from_str_radix(&r_string, 16);

            // G
            let mut g_string = "".to_string();
            if let Some(c) = chars.next() {
                g_string.push(c);
            }
            if let Some(c) = chars.next() {
                g_string.push(c);
            }

            let g = u8::from_str_radix(&g_string, 16);

            // B
            let mut b_string = "".to_string();
            if let Some(c) = chars.next() {
                b_string.push(c);
            }
            if let Some(c) = chars.next() {
                b_string.push(c);
            }

            let b = u8::from_str_radix(&b_string, 16);

            if r.is_ok() && g.is_ok() && b.is_ok() {
                let r = r.ok().unwrap();
                let g = g.ok().unwrap();
                let b = b.ok().unwrap();
                self.colors.push([r, g, b, 0xFF]);
                self.colors_f.push([r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0]);
            }
        }
    }

    /// Get the u8 based color at the given index
    pub fn at(&self, index: u8) -> [u8; 4] {
        self.colors[index as usize]
    }

    /// Get the f32 based color at the given index
    pub fn at_f(&self, index: u8) -> [f32; 4] {
        self.colors_f[index as usize]
    }

    /// Get the f32 based color at the given index
    pub fn at_vec(&self, index: u8) -> Vec3f {
        let c = self.colors_f[index as usize];
        Vec3f::new(c[0], c[1], c[2])
    }

    /// Get the f32 based color at the given index
    pub fn at_vec_to_linear(&self, index: u8) -> Vec3f {
        let mut c = self.colors_f[index as usize];
        c[0] = c[0].powf(2.2);
        c[1] = c[1].powf(2.2);
        c[2] = c[2].powf(2.2);
        Vec3f::new(c[0], c[1], c[2])
    }

    /// Get the f32 based color at the given index and convrts it to linear space
    pub fn at_f_to_linear(&self, index: u8) -> [f32; 4] {
        let mut c = self.colors_f[index as usize].clone();
        c[0] = c[0].powf(2.2);
        c[1] = c[1].powf(2.2);
        c[2] = c[2].powf(2.2);
        c
    }

    /// Returns the closest color index
    pub fn closest(&self, r: f32, g: f32, b: f32) -> u8 {
        let mut index = 0;
        let mut d = f32::MAX;

        for i in 0..self.colors_f.len() {

            // let dd =
            //     ((r - self.colors_f[i][0]) * 0.30).powf(2.0) +
            //     ((g - self.colors_f[i][1]) * 0.59).powf(2.0) +
            //     ((b - self.colors_f[i][2]) * 0.11).powf(2.0);

            let dd =
                ((r - self.colors_f[i][0]) * 1.0).powf(2.0) +
                ((g - self.colors_f[i][1]) * 1.0).powf(2.0) +
                ((b - self.colors_f[i][2]) * 1.0).powf(2.0);

            if dd < d {
                d = dd;
                index = i as u8;
            }
        }

        index
    }
}