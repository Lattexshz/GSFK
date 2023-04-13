#[cfg(target_os = "windows")]
pub use winey::platform::WindowExtForWindows as WindowExtForWindows;
#[cfg(target_os = "windows")]
pub use winey::platform::WindowCorner as WindowCorner;
#[cfg(target_os = "windows")]
pub use winey::platform::Margin as Margin;