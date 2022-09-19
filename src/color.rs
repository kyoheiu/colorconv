use crate::{
    color_names::{search_color_code, search_color_name},
    errors::IroError,
};

/// Color information.
pub struct Color {
    pub hex: String,
    pub name: Option<String>,
    pub rgb: [u8; 3],
    pub hsl: [f64; 3],
}

/// From hex code, or color name if exists.
impl TryFrom<&str> for Color {
    type Error = IroError;
    fn try_from(s: &str) -> Result<Self, IroError> {
        if let Some(code) = search_color_code(s) {
            Color::from_hex(&code)
        } else {
            Color::from_hex(s)
        }
    }
}

/// From rgb.
impl From<[u8; 3]> for Color {
    fn from(rgb: [u8; 3]) -> Self {
        let hex = format!("{}{}{}", to_hex(rgb[0]), to_hex(rgb[1]), to_hex(rgb[2]));
        let name = search_color_name(&hex);
        let hsl = convert_to_hsl(&rgb);
        Color {
            hex,
            name,
            rgb,
            hsl,
        }
    }
}

/// From hsl.
impl From<[f64; 3]> for Color {
    fn from(hsl: [f64; 3]) -> Self {
        let h = hsl::HSL {
            h: hsl[0],
            s: hsl[1],
            l: hsl[2],
        };
        let rgb = h.to_rgb();
        let rgb: [u8; 3] = [rgb.0, rgb.1, rgb.2];
        let hex = format!("{}{}{}", to_hex(rgb[0]), to_hex(rgb[1]), to_hex(rgb[2]));
        let name = search_color_name(&hex);
        Color {
            hex,
            name,
            rgb,
            hsl,
        }
    }
}

impl Color {
    fn from_hex(hex: &str) -> Result<Self, IroError> {
        let mut temp = "".to_string();
        let mut rgb_v = vec![];
        for (i, c) in hex.chars().enumerate() {
            if i % 2 != 0 {
                temp.push(c);
                rgb_v.push(temp);
                temp = "".to_string();
            } else {
                temp.push(c);
            }
        }

        let _strip = hex.to_string().strip_prefix('#');
        let hex = hex.chars().take(6).collect::<String>().to_ascii_lowercase();
        let name = search_color_name(&hex);

        let rgb: Vec<u8> = rgb_v
            .iter()
            .filter_map(|n| u8::from_str_radix(n, 16).ok())
            .collect();
        if rgb.len() != 3 {
            return Err(IroError("Cannot convert to u8.".to_string()));
        }
        let rgb: [u8; 3] = rgb.try_into().unwrap();

        let hsl = convert_to_hsl(&rgb);
        Ok(Color {
            hex,
            name,
            rgb,
            hsl,
        })
    }
}

fn round_f(n: f64) -> f64 {
    (n * 100.0).round() / 100.0
}

fn convert_to_hsl(rgb: &[u8]) -> [f64; 3] {
    let hsl = hsl::HSL::from_rgb(rgb);
    [round_f(hsl.h), round_f(hsl.s), round_f(hsl.l)]
}

fn to_hex(n: u8) -> String {
    let mut n = n;
    let mut i = 0;
    while n >= 16 {
        n -= 16;
        i += 1;
    }

    let mut result = "".to_string();
    result.push(map_16(i));
    result.push(map_16(n));
    result
}

fn map_16(n: u8) -> char {
    match n {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'a',
        11 => 'b',
        12 => 'c',
        13 => 'd',
        14 => 'e',
        _ => 'f',
    }
}
