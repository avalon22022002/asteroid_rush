pub mod laser {
    use macroquad::{
        color::{BLUE, RED, WHITE},
        math::Rect,
        texture::{FilterMode, Texture2D},
    };

    pub enum PrimaryColor {
        Blue,
        Red,
    }

    pub struct LaserTextureParams {
        pub primary_color: PrimaryColor,
    }

    // Every laser texture is this size, in pixels; only `primary_color`
    // varies between kinds. Private: callers read this off the `bounds`
    // that `build_laser_textures` returns, not the raw dimensions.
    const LASER_WIDTH: usize = 3;
    const LASER_HEIGHT: usize = 12;

    /// What `build_laser_textures` builds: the frames themselves, plus the
    /// `Rect` (anchored at the origin) bounding them.
    pub struct LaserTextures {
        pub textures: Vec<Texture2D>,
        pub bounds: Rect,
    }

    pub fn build_laser_textures(params: LaserTextureParams) -> LaserTextures {
        let width = LASER_WIDTH;
        let height = LASER_HEIGHT;

        let primary_color = match params.primary_color {
            PrimaryColor::Blue => BLUE,
            PrimaryColor::Red => RED,
        };

        let mut pixels = vec![0u8; width * height * 4];

        for y in 0..height {
            for x in 0..width {
                let i = (y * width + x) * 4;
                let color = if x == 1 { WHITE } else { primary_color };

                pixels[i] = (color.r * 255.0) as u8;
                pixels[i + 1] = (color.g * 255.0) as u8;
                pixels[i + 2] = (color.b * 255.0) as u8;
                pixels[i + 3] = (color.a * 255.0) as u8;
            }
        }

        let texture = Texture2D::from_rgba8(width as u16, height as u16, &pixels);
        texture.set_filter(FilterMode::Nearest);

        LaserTextures {
            textures: vec![texture],
            bounds: Rect::new(0.0, 0.0, width as f32, height as f32),
        }
    }
}
