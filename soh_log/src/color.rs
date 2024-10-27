#[allow(dead_code)]

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
}

pub struct AnsiFlags {
    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,
    pub fg_bright: bool,
    pub bg_bright: bool,

    pub bold: bool,
    pub italic: bool,
}

impl AnsiFlags {
    pub const fn get_ansi_default() -> &'static [u8] {
        return b"\x1b[0m";
    }

    pub fn to_ansi_string(&self) -> Vec<u8> {
        let mut res = vec![33, b'['];

        if self.bold == true {
            res.push(b'1');
            res.push(b';');
        }
        if self.italic == true {
            res.push(b'3');
            res.push(b';');
        }

        if let Some(fg_color) = self.fg_color {
            if self.fg_bright == false {
                res.push(b'3');
            } else {
                res.push(b'9');
            }

            res.push(b'0' + fg_color as u8);
            res.push(b';');
        }

        if let Some(bg_color) = self.bg_color {
            if self.fg_bright == false {
                res.push(b'4');
            } else {
                res.push(b'1');
                res.push(b'0');
            }

            res.push(b'0' + bg_color as u8);
            res.push(b';');
        }

        res.push(b'm');

        return res;
    }
}

impl Default for AnsiFlags {
    fn default() -> Self {
        return AnsiFlags {
            fg_color: None,
            bg_color: None,
            fg_bright: false,
            bg_bright: false,
            bold: false,
            italic: false,
        };
    }
}
