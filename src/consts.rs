use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static!{
    pub static ref COLORS: HashMap<char, egui::Color32> = [
        ('a', egui::Color32::from_rgb(204, 0, 1)),
        ('b', egui::Color32::from_rgb(251, 148, 11)),
        ('c', egui::Color32::from_rgb(255, 255, 1)),
        ('d', egui::Color32::from_rgb(1, 204, 0)),
        ('e', egui::Color32::from_rgb(3, 192, 198)),
        ('f', egui::Color32::from_rgb(0, 0, 254)),
        ('g', egui::Color32::from_rgb(118, 44, 167)),
        ('h', egui::Color32::from_rgb(254, 152, 191)),
        ('x', egui::Color32::DARK_GRAY),
        ('y', egui::Color32::BLACK),
        ('z', egui::Color32::WHITE),
    ].iter().copied().collect();
}
