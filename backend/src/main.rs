// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    Manager,
    tray::{TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState},
    menu::{Menu, MenuItem},
};
use dot::{AppState, commands};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Initialize app state
            let app_state = AppState::new();
            app.manage(app_state);

            // Create tray menu
            let show_item = MenuItem::with_id(app, "show", "Show dot", true, None::<&str>)?;
            let separator = MenuItem::with_id(app, "separator", "---", false, None::<&str>)?;
            let feed_item = MenuItem::with_id(app, "feed", "Feed", true, None::<&str>)?;
            let play_item = MenuItem::with_id(app, "play", "Play", true, None::<&str>)?;
            let sleep_item = MenuItem::with_id(app, "sleep", "Sleep", true, None::<&str>)?;
            let separator2 = MenuItem::with_id(app, "separator2", "---", false, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

            let menu = Menu::with_items(
                app,
                &[&show_item, &separator, &feed_item, &play_item, &sleep_item, &separator2, &quit_item],
            )?;

            // Create tray icon
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(move |app, event| {
                    match event.id.as_ref() {
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "feed" => {
                            if let Some(state) = app.try_state::<AppState>() {
                                if let Ok(mut pet) = state.pet.lock() {
                                    pet.feed();
                                }
                            }
                        }
                        "play" => {
                            if let Some(state) = app.try_state::<AppState>() {
                                if let Ok(mut pet) = state.pet.lock() {
                                    pet.play();
                                }
                            }
                        }
                        "sleep" => {
                            if let Some(state) = app.try_state::<AppState>() {
                                if let Ok(mut pet) = state.pet.lock() {
                                    pet.sleep();
                                }
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_pet_state,
            commands::feed_pet,
            commands::play_with_pet,
            commands::put_to_sleep,
            commands::revive_pet,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
