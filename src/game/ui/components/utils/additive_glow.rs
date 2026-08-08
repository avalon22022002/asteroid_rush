//! Shared additive-blend pass for "glow on hover" effects.
//!
//! Drawing a sprite through this *adds* its light to the framebuffer
//! (`dst += srcAlpha * src`) instead of replacing pixels, so bright areas glow
//! and dark areas stay put — a blue UI panel lights up while its dark bevel
//! doesn't, with no mask or separate sprite.

use std::cell::OnceCell;

use macroquad::{
    miniquad::{BlendFactor, BlendState, BlendValue, Equation, PipelineParams},
    prelude::*,
};

// macroquad's default texture shaders, unchanged — only the blend mode differs.
// `color0` arrives 0..255 and is normalized to match `draw_texture_ex`'s tint.
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
/// controls glow strength; its rgb biases the added light's hue.
pub fn draw(texture: &Texture2D, bounds: Rect, tint: Color) {
    gl_use_material(&material());
    draw_texture_ex(
        texture,
        bounds.x,
        bounds.y,
        tint,
        DrawTextureParams {
            dest_size: Some(bounds.size()),
            ..Default::default()
        },
    );
    gl_use_default_material();
}
