//! Shared "glow on hover" pass.
//!
//! Draws the sprite a second time in "add light" mode — like stacking a glowing
//! copy on top instead of painting over it. Bright parts pile on more light and
//! glow; dark or see-through parts have nothing to add and stay the same. So a
//! blue panel lights up while its dark border doesn't — no mask or extra sprite.
//! (The GPU does this as `result = src * srcAlpha + dst`.)

use std::cell::OnceCell;

use macroquad::{
    miniquad::{BlendFactor, BlendState, BlendValue, Equation, PipelineParams},
    prelude::*,
};

// macroquad's default texture shaders, unchanged — only the blend mode (set in
// `material`) makes this additive. `color0` is the 0..255 tint, normalized here.
const VERTEX_SHADER: &str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
varying lowp vec2 uv;
varying lowp vec4 color;
uniform mat4 Model;
uniform mat4 Projection;
void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    color = color0 / 255.0;
    uv = texcoord;
}";

const FRAGMENT_SHADER: &str = "#version 100
varying lowp vec4 color;
varying lowp vec2 uv;
uniform sampler2D Texture;
void main() {
    gl_FragColor = color * texture2D(Texture, uv);
}";

thread_local! {
    // Compiled lazily on first use (needs a live GL context) and reused
    // everywhere. macroquad is single-threaded, so thread-local is safe.
    static MATERIAL: OnceCell<Material> = const { OnceCell::new() };
}

fn material() -> Material {
    MATERIAL.with(|cell| {
        cell.get_or_init(|| {
            load_material(
                ShaderSource::Glsl {
                    vertex: VERTEX_SHADER,
                    fragment: FRAGMENT_SHADER,
                },
                MaterialParams {
                    pipeline_params: PipelineParams {
                        // Additive blend: result = src * srcAlpha + dst. This is
                        // what makes the draw add light instead of covering.
                        color_blend: Some(BlendState::new(
                            Equation::Add,
                            BlendFactor::Value(BlendValue::SourceAlpha),
                            BlendFactor::One,
                        )),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            )
            .unwrap_or_else(|e| panic!("[additive_glow] failed to build material: {e}"))
        })
        .clone()
    })
}

/// Draws `texture` at `bounds` additively tinted by `tint`. The tint's alpha
/// controls glow strength; its rgb biases the added light's hue. `source` crops
/// the sampled texels — pass the same crop the base art uses so the glow lines
/// up; `None` samples the whole texture.
pub fn draw(texture: &Texture2D, bounds: Rect, tint: Color, source: Option<Rect>) {
    gl_use_material(&material());
    draw_texture_ex(
        texture,
        bounds.x,
        bounds.y,
        tint,
        DrawTextureParams {
            dest_size: Some(bounds.size()),
            source,
            ..Default::default()
        },
    );
    gl_use_default_material();
}
