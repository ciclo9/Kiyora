use std::cell::RefCell;
use tray_icon::{
    menu::{Menu, MenuItem, MenuId},
    Icon, TrayIcon, TrayIconBuilder,
};
use log::{info, warn};

thread_local! {
    // The icon must be touched only on the thread that created it (the UI thread).
    static TRAY_ICON: RefCell<Option<TrayIcon>> = RefCell::new(None);
}

/// Sets the tray tooltip from any thread. Runs on the UI thread only when called,
/// so nothing polls for tooltip changes.
pub fn update_tray_tooltip(tooltip: &str) {
    let tip = tooltip.to_string();
    let _ = slint::invoke_from_event_loop(move || {
        TRAY_ICON.with(|icon| {
            if let Some(icon) = icon.borrow().as_ref() {
                let _ = icon.set_tooltip(Some(tip));
            }
        });
    });
}

pub struct SystemTrayManager {
    pub show_item_id: MenuId,
    pub quit_item_id: MenuId,
}


fn create_default_icon() -> Icon {
    let icon_bytes = include_bytes!("../../assets/app_icon.png");
    if let Ok(img) = image::load_from_memory(icon_bytes) {
        let rgba_img = img.into_rgba8();
        let (width, height) = rgba_img.dimensions();
        if let Ok(icon) = Icon::from_rgba(rgba_img.into_raw(), width, height) {
            return icon;
        }
    }

    // Fallback: Kiyora cyan (#3CD7EB) on deep slate (#141A23)
    let width = 32u32;
    let height = 32u32;
    let mut rgba = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            let is_border = x == 0 || x == width - 1 || y == 0 || y == height - 1;
            if is_border {
                // Deep slate border: #141A23
                rgba.extend_from_slice(&[0x14, 0x1A, 0x23, 0xFF]);
            } else {
                // Kiyora glacial cyan: #3CD7EB
                rgba.extend_from_slice(&[0x3C, 0xD7, 0xEB, 0xFF]);
            }
        }
    }

    Icon::from_rgba(rgba, width, height).expect("Failed to generate Kiyora tray icon")
}

impl SystemTrayManager {
    /// Call on the UI thread.
    pub fn setup() -> Self {
        let tray_menu = Menu::new();
        let show_item = MenuItem::new("Show Kiyora", true, None);
        let quit_item = MenuItem::new("Quit", true, None);

        let show_id = show_item.id().clone();
        let quit_id = quit_item.id().clone();

        let _ = tray_menu.append(&show_item);
        let _ = tray_menu.append(&quit_item);

        let icon = create_default_icon();

        let tray_icon = match TrayIconBuilder::new()
            .with_icon(icon)
            .with_menu(Box::new(tray_menu))
            .with_menu_on_left_click(true)
            .with_tooltip("Kiyora — Native Discord Client")
            .build()
        {
            Ok(icon) => {
                info!("Kiyora tray icon activated in System Tray.");
                Some(icon)
            }
            Err(e) => {
                warn!("Could not initialize System Tray icon ({:?}), running in window mode.", e);
                None
            }
        };

        TRAY_ICON.with(|icon| *icon.borrow_mut() = tray_icon);

        Self {
            show_item_id: show_id,
            quit_item_id: quit_id,
        }
    }
}
