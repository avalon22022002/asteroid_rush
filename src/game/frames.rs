//! Building and decoding frame lists for sprite/animation loading:
//! `frame_sequence!` bakes `(path, bytes)` pairs in at compile time via
//! `include_bytes!`, and `load_frames` decodes them into textures at
//! runtime.

use macroquad::texture::Texture2D;

const LOG_PREFIX: &str = "[frames]";

/// Expands to a `&[(&str, &[u8])]` frame list: one entry per `suffix`,
/// built from `concat!($base, suffix, ".png")`.
///
/// # Example
///
/// ```ignore
/// // `base` is always relative to the project root (CARGO_MANIFEST_DIR),
/// // regardless of which file invokes the macro.
/// let alive = load_frames(frame_sequence!(
///     "assets/animations/ships/sentinel/sentinel_",
///     ["00", "01", "02", "03", "04", "05", "06", "07"]
/// ))
/// .await;
/// ```
macro_rules! frame_sequence {
    ($base:literal, [$($suffix:literal),+ $(,)?]) => {
        &[$((
            concat!($base, $suffix, ".png"),
            include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/",
                $base,
                $suffix,
                ".png"
            )),
        )),+]
    };
}
// `macro_rules!` macros aren't visible outside their defining file by
// default (unlike normal items). This re-export makes `frame_sequence`
// importable via a regular `use` path (e.g. from `entities::ship`).
pub(crate) use frame_sequence;

/// Decodes each `(path, bytes)` pair from a `frame_sequence!`-built slice
/// into a texture, logging progress. `path` is only used for load logging
/// — pass the same path given to `include_bytes!` at the call site so log
/// output (and the panic message, if the bytes aren't a valid image) names
/// the actual file.
///
/// `async` here matches the asset-loading convention used elsewhere (e.g.
/// `AudioRepository::load_all`) so this composes with `.await` chains,
/// even though the bytes are already resident (via `include_bytes!`) and
/// decoding itself is synchronous.
pub async fn load_frames(frames: &[(&str, &[u8])]) -> Vec<Texture2D> {
    println!(
        "{LOG_PREFIX} loading {} frame(s)...",
        frames.len()
    );

    let frames = frames
        .iter()
        .map(|(path, bytes)| {
            println!("{LOG_PREFIX} loading {path}...");
            Texture2D::from_file_with_format(bytes, None)
        })
        .collect();

    println!("{LOG_PREFIX} load complete");

    frames
}
