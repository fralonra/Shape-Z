#[derive(PartialEq, Debug, Clone)]
pub struct Palette {

    pub colors                      : Vec<[u8; 4]>,
}

impl Palette {
    pub fn new() -> Self {
        Self {
            colors                  : vec![],
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
                self.colors.push([r.ok().unwrap(), g.ok().unwrap(), b.ok().unwrap(), 0xFF]);
            }
        }

    }
}