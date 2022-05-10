#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent};
use tauri::Manager;

use cocoa::appkit::{NSWindow, NSWindowStyleMask};
use tauri::{Runtime, Window};

pub trait WindowExt {
  #[cfg(target_os = "macos")]
  fn set_transparent_titlebar(&self, transparent: bool);
}

impl<R: Runtime> WindowExt for Window<R> {
  #[cfg(target_os = "macos")]
  fn set_transparent_titlebar(&self, transparent: bool) {
    use cocoa::appkit::NSWindowTitleVisibility;

    unsafe {
      let id = self.ns_window().unwrap() as cocoa::base::id;

      let mut style_mask = id.styleMask();
      style_mask.set(
        NSWindowStyleMask::NSFullSizeContentViewWindowMask,
        transparent,
      );
      id.setStyleMask_(style_mask);

      id.setTitleVisibility_(if transparent {
        NSWindowTitleVisibility::NSWindowTitleHidden
      } else {
        NSWindowTitleVisibility::NSWindowTitleVisible
      });
      id.setTitlebarAppearsTransparent_(if transparent {
        cocoa::base::YES
      } else {
        cocoa::base::NO
      });
    }
  }
}

fn main() {
  let tray_menu = SystemTrayMenu::new()
    .add_item(CustomMenuItem::new("hide".to_string(), "Hide"))
    .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));

  let system_tray = SystemTray::new()
    .with_menu(tray_menu);

  tauri::Builder::default()
    .system_tray(system_tray)
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::MenuItemClick { id, .. } => {
        match id.as_str() {
          "quit" => {
            std::process::exit(0);
          }
          "hide" => {
            let window = app.get_window("main").unwrap();
            window.hide().unwrap();
          }
          _ => {}
        }
      }
      _ => {}
    })
    .setup(|app| {
      let win = app.get_window("main").unwrap();
      win.set_transparent_titlebar(true);

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
