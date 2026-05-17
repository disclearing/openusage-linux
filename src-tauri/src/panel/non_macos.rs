use std::sync::Once;

use tauri::{AppHandle, Manager, Position, Size, WebviewWindow, WindowEvent};

static INIT: Once = Once::new();

fn monitor_contains_physical_point(
    origin_x: f64,
    origin_y: f64,
    width: f64,
    height: f64,
    point_x: f64,
    point_y: f64,
) -> bool {
    point_x >= origin_x
        && point_x < origin_x + width
        && point_y >= origin_y
        && point_y < origin_y + height
}

fn main_window(app_handle: &AppHandle) -> Option<WebviewWindow> {
    app_handle.get_webview_window("main")
}

fn position_panel_from_tray(app_handle: &AppHandle) {
    let Some(tray) = app_handle.tray_by_id("tray") else {
        log::debug!("position_panel_from_tray: tray icon not found");
        position_panel_fallback(app_handle);
        return;
    };
    match tray.rect() {
        Ok(Some(rect)) => {
            position_panel_at_tray_icon(app_handle, rect.position, rect.size);
        }
        Ok(None) => {
            log::debug!("position_panel_from_tray: tray rect not available yet");
            position_panel_fallback(app_handle);
        }
        Err(e) => {
            log::warn!("position_panel_from_tray: failed to get tray rect: {}", e);
            position_panel_fallback(app_handle);
        }
    }
}

fn position_panel_fallback(app_handle: &AppHandle) {
    let Some(window) = main_window(app_handle) else {
        return;
    };

    let monitor = match window.primary_monitor() {
        Ok(Some(m)) => m,
        Ok(None) => return,
        Err(e) => {
            log::warn!("fallback positioning: failed to read primary monitor: {}", e);
            return;
        }
    };

    let scale = monitor.scale_factor();
    let mon_x = monitor.position().x as f64 / scale;
    let mon_y = monitor.position().y as f64 / scale;
    let mon_w = monitor.size().width as f64 / scale;

    let (panel_w, _panel_h) = match (window.outer_size(), window.scale_factor()) {
        (Ok(s), Ok(win_scale)) => (s.width as f64 / win_scale, s.height as f64 / win_scale),
        _ => {
            let conf: serde_json::Value = serde_json::from_str(include_str!("../../tauri.conf.json"))
                .expect("tauri.conf.json must be valid JSON");
            let width = conf["app"]["windows"][0]["width"]
                .as_f64()
                .expect("width must be set in tauri.conf.json");
            let height = conf["app"]["windows"][0]["height"]
                .as_f64()
                .expect("height must be set in tauri.conf.json");
            (width, height)
        }
    };

    let margin_x = 18.0;
    let margin_y = 18.0;
    let x = mon_x + mon_w - panel_w - margin_x;
    let y = mon_y + margin_y;
    let _ = window.set_position(Position::Logical(tauri::LogicalPosition { x, y }));
}

pub fn init(app_handle: &AppHandle) -> tauri::Result<()> {
    if let Some(window) = main_window(app_handle) {
        let _ = window.set_always_on_top(true);
        let _ = window.set_skip_taskbar(true);

        INIT.call_once(|| {
            let window_clone = window.clone();
            window.on_window_event(move |event| {
                if let WindowEvent::Focused(false) = event {
                    let _ = window_clone.hide();
                }
            });
        });
    }

    Ok(())
}

pub fn show_panel(app_handle: &AppHandle) {
    if let Err(err) = init(app_handle) {
        log::error!("Failed to init panel: {}", err);
        return;
    }

    let Some(window) = main_window(app_handle) else {
        log::error!("Main window missing");
        return;
    };

    let _ = window.show();
    let _ = window.set_focus();
    position_panel_from_tray(app_handle);
}

pub fn hide_panel(app_handle: &AppHandle) {
    if let Some(window) = main_window(app_handle) {
        let _ = window.hide();
    }
}

pub fn toggle_panel(app_handle: &AppHandle) {
    if let Err(err) = init(app_handle) {
        log::error!("Failed to init panel: {}", err);
        return;
    }

    let Some(window) = main_window(app_handle) else {
        return;
    };

    match window.is_visible() {
        Ok(true) => {
            log::debug!("toggle_panel: hiding panel");
            let _ = window.hide();
        }
        Ok(false) => {
            log::debug!("toggle_panel: showing panel");
            let _ = window.show();
            let _ = window.set_focus();
            position_panel_from_tray(app_handle);
        }
        Err(e) => {
            log::warn!("toggle_panel: failed to read visibility: {}", e);
        }
    }
}

pub fn handle_tray_click(app_handle: &AppHandle, icon_position: Position, icon_size: Size) {
    if let Err(err) = init(app_handle) {
        log::error!("Failed to init panel: {}", err);
        return;
    }

    let Some(window) = main_window(app_handle) else {
        return;
    };

    match window.is_visible() {
        Ok(true) => {
            log::debug!("tray click: hiding panel");
            let _ = window.hide();
        }
        Ok(false) => {
            log::debug!("tray click: showing panel");
            let _ = window.show();
            let _ = window.set_focus();
            position_panel_at_tray_icon(app_handle, icon_position, icon_size);
        }
        Err(e) => {
            log::warn!("tray click: failed to read visibility: {}", e);
        }
    }
}

pub fn set_always_on_top(app_handle: &AppHandle, pinned: bool) -> Result<(), String> {
    let Some(window) = main_window(app_handle) else {
        return Err("Main window missing".to_string());
    };
    window
        .set_always_on_top(pinned)
        .map_err(|e| format!("failed to set always-on-top: {}", e))
}

pub fn position_panel_at_tray_icon(app_handle: &AppHandle, icon_position: Position, icon_size: Size) {
    let Some(window) = main_window(app_handle) else {
        return;
    };

    let (icon_phys_x, icon_phys_y) = match &icon_position {
        Position::Physical(pos) => (pos.x as f64, pos.y as f64),
        Position::Logical(pos) => (pos.x, pos.y),
    };
    let (icon_phys_w, icon_phys_h) = match &icon_size {
        Size::Physical(s) => (s.width as f64, s.height as f64),
        Size::Logical(s) => (s.width, s.height),
    };

    let monitors = match window.available_monitors() {
        Ok(monitors) => monitors,
        Err(e) => {
            log::warn!("failed to read monitors for panel positioning: {}", e);
            return;
        }
    };

    let icon_center_x = icon_phys_x + (icon_phys_w / 2.0);
    let icon_center_y = icon_phys_y + (icon_phys_h / 2.0);

    let found_monitor = monitors.iter().find(|monitor| {
        let origin = monitor.position();
        let size = monitor.size();
        monitor_contains_physical_point(
            origin.x as f64,
            origin.y as f64,
            size.width as f64,
            size.height as f64,
            icon_center_x,
            icon_center_y,
        )
    });

    let monitor = match found_monitor {
        Some(m) => m.clone(),
        None => {
            log::warn!(
                "No monitor found for tray rect center at ({:.0}, {:.0}), using primary",
                icon_center_x,
                icon_center_y
            );
            match window.primary_monitor() {
                Ok(Some(m)) => m,
                _ => return,
            }
        }
    };

    let target_scale = monitor.scale_factor();
    let mon_phys_x = monitor.position().x as f64;
    let mon_phys_y = monitor.position().y as f64;
    let mon_logical_x = mon_phys_x / target_scale;
    let mon_logical_y = mon_phys_y / target_scale;

    let icon_logical_x = mon_logical_x + (icon_phys_x - mon_phys_x) / target_scale;
    let icon_logical_y = mon_logical_y + (icon_phys_y - mon_phys_y) / target_scale;
    let icon_logical_w = icon_phys_w / target_scale;
    let icon_logical_h = icon_phys_h / target_scale;

    let panel_width = match (window.outer_size(), window.scale_factor()) {
        (Ok(s), Ok(win_scale)) => s.width as f64 / win_scale,
        _ => {
            let conf: serde_json::Value = serde_json::from_str(include_str!("../../tauri.conf.json"))
                .expect("tauri.conf.json must be valid JSON");
            conf["app"]["windows"][0]["width"]
                .as_f64()
                .expect("width must be set in tauri.conf.json")
        }
    };

    let icon_center_x = icon_logical_x + (icon_logical_w / 2.0);
    let panel_x = icon_center_x - (panel_width / 2.0);
    let nudge_y: f64 = 6.0;
    let panel_y = icon_logical_y + icon_logical_h + nudge_y;

    let _ = window.set_position(Position::Logical(tauri::LogicalPosition {
        x: panel_x,
        y: panel_y,
    }));
}
