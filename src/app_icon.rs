//! Shared window icon. Generated from assets/snor.svg alongside the .ico.

pub fn data() -> eframe::egui::IconData {
    eframe::egui::IconData {
        rgba: include_bytes!("../assets/snor-64.rgba").to_vec(),
        width: 64,
        height: 64,
    }
}
