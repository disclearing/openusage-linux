#[cfg(target_os = "macos")]
mod macos;
#[cfg(not(target_os = "macos"))]
mod non_macos;

use tauri::{AppHandle, Position, Size};

#[cfg(target_os = "macos")]
pub fn init(app_handle: &AppHandle) -> tauri::Result<()> {
    macos::init(app_handle)
}

#[cfg(not(target_os = "macos"))]
pub fn init(app_handle: &AppHandle) -> tauri::Result<()> {
    non_macos::init(app_handle)
}

#[cfg(target_os = "macos")]
pub fn show_panel(app_handle: &AppHandle) {
    macos::show_panel(app_handle);
}

#[cfg(not(target_os = "macos"))]
pub fn show_panel(app_handle: &AppHandle) {
    non_macos::show_panel(app_handle);
}

#[cfg(target_os = "macos")]
pub fn hide_panel(app_handle: &AppHandle) {
    macos::hide_panel(app_handle);
}

#[cfg(not(target_os = "macos"))]
pub fn hide_panel(app_handle: &AppHandle) {
    non_macos::hide_panel(app_handle);
}

#[cfg(target_os = "macos")]
pub fn toggle_panel(app_handle: &AppHandle) {
    macos::toggle_panel(app_handle);
}

#[cfg(not(target_os = "macos"))]
pub fn toggle_panel(app_handle: &AppHandle) {
    non_macos::toggle_panel(app_handle);
}

#[cfg(target_os = "macos")]
pub fn handle_tray_click(app_handle: &AppHandle, icon_position: Position, icon_size: Size) {
    macos::handle_tray_click(app_handle, icon_position, icon_size);
}

#[cfg(not(target_os = "macos"))]
pub fn handle_tray_click(app_handle: &AppHandle, icon_position: Position, icon_size: Size) {
    non_macos::handle_tray_click(app_handle, icon_position, icon_size);
}

#[cfg(target_os = "macos")]
pub fn set_always_on_top(_app_handle: &AppHandle, _pinned: bool) -> Result<(), String> {
    Ok(())
}

#[cfg(not(target_os = "macos"))]
pub fn set_always_on_top(app_handle: &AppHandle, pinned: bool) -> Result<(), String> {
    non_macos::set_always_on_top(app_handle, pinned)
}
