//! Shared "glow on mouse hover".
//!
//! Draws the sprite a second time in **"add light" mode** on top of the one
//! already on screen, which makes its bright parts glow.
//!
//! # Paint vs. light
//!
//! There are two ways to combine what you draw with what's already there:
//!
//! - **Normal mode = paint.** New pixels *replace* what's underneath, like a
//!   sticker. Draw grey over black → you see grey; the black is gone.
//! - **Add-light mode = flashlights.** New pixels' brightness is *added* to
//!   what's there, like two flashlights on one spot — nothing is covered, things
//!   only get brighter. (Red light + green light = brighter yellow. Red paint +
//!   green paint = darker mud. Light adds; paint doesn't.)
//!
//! # Why this glows only the bright parts
//!
//! Drawing the button a second time in add-light mode adds *each pixel's own
//! brightness* back onto itself:
//!
//! - **Bright blue panel** → already bright, add more → clearly brighter (glow).
//! - **Dark grey border** → nearly black, adds ~nothing → looks unchanged.
//! - **Transparent margin** → adds nothing.
//!
//! So the glow follows the art's bright areas for free — no mask or extra
//! sprite. Formally the GPU computes `result = drawn * drawnAlpha + existing`.
//!
//! # How it's wired
//!
//! "Add light" is a GPU *blend mode*. macroquad draws with normal (paint)
//! blending by default, so to switch we hand it a [`Material`] — a small bundle
//! of "how to draw." Ours reuses macroquad's stock texture shaders and only
//! swaps the blend mode to additive (see `additive_blend`). Everything else in
//! this file just builds that material once and reuses it.

use std::cell::OnceCell;

use macroquad::{
    miniquad::{BlendFactor, BlendState, BlendValue, Equation, PipelineParams},
    prelude::*,
};

/// Draws `texture` at `bounds` in additive "add light" mode. The tint's alpha
/// sets glow strength; its rgb biases the added light's hue. `source` crops the
/// sampled texels — pass the same crop the base art uses so the glow lines up;
/// `None` samples the whole texture.
pub fn draw(texture: &Texture2D, bounds: Rect, tint: Color, source: Option<Rect>) {
    // Switch to additive drawing, draw the sprite, then switch back so nothing
    // else in the frame is affected.
    gl_use_material(&glow_material());
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

/// The additive material, built once and reused. It's cached because building
/// it needs a live GL context (so it can't exist until the window does) and we
/// only ever need one. Thread-local is enough — macroquad runs on one thread.
fn glow_material() -> Material {
    thread_local! {
        static CACHED: OnceCell<Material> = const { OnceCell::new() };
    }
    CACHED.with(|cache| cache.get_or_init(build_glow_material).clone())
}

/// Builds the material: macroquad's default texture shaders, with only the
/// blend mode changed to additive.
fn build_glow_material() -> Material {
    let params = MaterialParams {
        pipeline_params: PipelineParams {
            color_blend: Some(additive_blend()),
            ..Default::default()
        },
        ..Default::default()
    };
    load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER,
        },
        params,
    )
    .expect("[additive_glow] failed to build material")
}

/// The one line that makes this "add light": each drawn pixel is added on top of
/// what's already there — `result = drawn * drawnAlpha + existing` — instead of
/// replacing it.
fn additive_blend() -> BlendState {
    BlendState::new(
        Equation::Add,
        BlendFactor::Value(BlendValue::SourceAlpha),
        BlendFactor::One,
    )
}

// Boilerplate below: these are macroquad's stock texture shaders, copied
// verbatim. They just position the sprite and output `tint * texture` per pixel
// — the interesting part is the blend mode above, not these. `color0` is the
// 0..255 tint, normalized to 0..1 here.
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
