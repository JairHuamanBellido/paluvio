use gpui_kit::{Hsla, Rgba};

/// HSV coordinates, normalized to 0..=1. Brightness is HSV value, not HSL lightness.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsv {
    pub hue: f32,
    pub saturation: f32,
    pub brightness: f32,
}

impl Hsv {
    pub fn new(hue: f32, saturation: f32, brightness: f32) -> Self {
        Self {
            hue: hue.clamp(0., 1.),
            saturation: saturation.clamp(0., 1.),
            brightness: brightness.clamp(0., 1.),
        }
    }

    pub fn from_color(color: Hsla) -> Self {
        let rgb = Rgba::from(color);
        let maximum = rgb.r.max(rgb.g).max(rgb.b);
        let minimum = rgb.r.min(rgb.g).min(rgb.b);
        let saturation = if maximum == 0. {
            0.
        } else {
            (maximum - minimum) / maximum
        };
        Self::new(color.h, saturation, maximum)
    }

    pub fn to_color(self) -> Hsla {
        let channel = |offset: f32| {
            let k = (offset + self.hue * 6.) % 6.;
            self.brightness * (1. - self.saturation * k.min(4. - k).clamp(0., 1.))
        };
        Rgba {
            r: channel(5.),
            g: channel(3.),
            b: channel(1.),
            a: 1.,
        }
        .into()
    }

    pub fn rust_value(self, name: &str) -> String {
        format!(
            "let {name} = Hsv::new({:.8}, {:.8}, {:.8});",
            self.hue, self.saturation, self.brightness
        )
    }

    /// Round channels so displaying an entered hex color preserves its bytes.
    pub fn to_hex(self) -> String {
        let color = Rgba::from(self.to_color());
        format!(
            "#{:02X}{:02X}{:02X}",
            (color.r * 255.).round() as u8,
            (color.g * 255.).round() as u8,
            (color.b * 255.).round() as u8
        )
    }
}
