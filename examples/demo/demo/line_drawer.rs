pub mod douglas_peucker_drawer;
pub mod original_drawer;

use std::ops::RangeInclusive;

pub use douglas_peucker_drawer::DouglasPeuckerDrawer;
pub use original_drawer::OriginalDrawer;

use eframe::egui::{DragValue, Painter, Ui};
use emath::{Pos2, RectTransform};

const THRESHOLD_DEFAULT: f32 = 0.05;
const THRESHOLD_LIMIT: f32 = 0.5;
const THRESHOLD_DRAG_VALUE: f32 = 0.002;
const THRESHOLD_INCREMENT: f32 = 0.005;

pub trait LineDrawer {
    fn draw_settings(&mut self, ui: &mut Ui);
    fn draw_lines(&mut self, to_screen: &RectTransform, painter: &Painter);
    fn add_line(&mut self, line: &Vec<Pos2>);
    fn clear(&mut self);
}

pub fn draw_threshold_setting(ui: &mut Ui, threshold: &mut f32, text: &str) {
    ui.horizontal(|ui| {
        ui.add(
            DragValue::new(threshold)
                .speed(THRESHOLD_DRAG_VALUE)
                .clamp_range(RangeInclusive::new(0.0, THRESHOLD_LIMIT)),
        );
        if ui.button("+").clicked() {
            *threshold += THRESHOLD_INCREMENT;
        }
        if ui.button("-").clicked() {
            *threshold -= THRESHOLD_INCREMENT;
        }
        ui.label(text);
    });
}
