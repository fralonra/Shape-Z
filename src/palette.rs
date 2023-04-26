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
}