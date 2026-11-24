mod agent_adapter;
mod library_scanner;
mod skill_parser;

use agent_adapter::ClaudeCodeAdapter;
use library_scanner::{scan_library, Skill};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, WindowEvent};
use tauri_plugin_positioner::{Position, WindowExt};

const PANEL: &str = "panel";

#[tauri::command]
fn list_skills(app: AppHandle) -> Result<Vec<Skill>, String> {
    let home = app.path().home_dir().map_err(|e| e.to_string())?;
    Ok(scan_library(&ClaudeCodeAdapter::for_home(home)))
}

fn toggle_panel(app: &AppHandle) {
    let Some(panel) = app.get_webview_window(PANEL) else { return };
    if panel.is_visible().unwrap_or(false) {
        let _ = panel.hide();
    } else {
        let _ = panel.move_window(Position::TrayCenter);
        let _ = panel.show();
        let _ = panel.set_focus();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_positioner::init())
        .invoke_handler(tauri::generate_handler![list_skills])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            TrayIconBuilder::new()
                .icon(app.default_window_icon().cloned().expect("bundle has an icon"))
                .tooltip("Skill Shelf")
                .on_tray_icon_event(|tray, event| {
                    tauri_plugin_positioner::on_tray_event(tray.app_handle(), &event);
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        toggle_panel(tray.app_handle());
                    }
                })
                .build(app)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() == PANEL {
                if let WindowEvent::Focused(false) = event {
                    let _ = window.hide();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
