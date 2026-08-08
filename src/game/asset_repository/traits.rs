use std::sync::OnceLock;

/// A repository that loads all its assets up front, rather than lazily.
pub trait Preloadable {
    /// Loads everything. Call once at init so decoding cost lands there,
    /// not mid-gameplay. Must be idempotent — repeat calls are no-ops. If
    /// loading multiple independent things, prefer `futures::join!` over
    /// awaiting them one after another, so the wait is the slowest single
    /// load instead of their sum.
    ///
    /// Written as `-> impl Future<Output = ()>` instead of `async fn` so
    /// implementers can bound the future (e.g. `+ Send`) if needed —
    /// equivalent to `async fn load_all(&mut self)` otherwise.
    fn load_all(&mut self) -> impl Future<Output = ()>;
}

/// A type with one shared instance, built lazily on first use.
pub trait Singleton: Sized + 'static {
    /// This type's storage cell. Implementers declare
    /// `static INSTANCE: OnceLock<Self> = OnceLock::new();` and return it.
    /// `get_instance` only hands out `&Self`, never `&mut Self` — mutating
    /// fields afterward needs their own interior mutability (`Mutex`,
    /// `RefCell`, `OnceLock`, ...).
    fn storage() -> &'static OnceLock<Self>;

    /// Builds a fresh instance. Called once, the first time `get_instance`
    /// runs.
    fn new() -> Self;

    /// Returns the shared instance, building it via `new` on first call.
    /// Every call after that is a cheap lookup.
    fn get_instance() -> &'static Self {
        Self::storage().get_or_init(Self::new)
    }
}