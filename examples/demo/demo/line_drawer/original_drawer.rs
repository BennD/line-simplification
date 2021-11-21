use eframe::egui;
use egui::{Color32, Stroke};
use emath::{Pos2, RectTransform};

use super::LineDrawer;

const LINE_COLOR: Color32 = Color32::GRAY;
const LINE_WIDTH: f32 = 4.0;

pub struct OriginalDrawer {
    stroke: Stroke,
    lines: Vec<Vec<Pos2>>,
}

impl Default for OriginalDrawer {
    fn default() -> Self {
        Self {
            stroke: Stroke::new(LINE_WIDTH, LINE_COLOR),
            lines: vec![],
        }
    }
}

impl LineDrawer for OriginalDrawer {
    fn draw_settings(&mut self, ui: &mut eframe::egui::Ui) {
        ui.group(|ui| {
            ui.heading("Original Line");
            egui::stroke_ui(ui, &mut self.stroke, "Stroke");
        });
    }

    fn draw_lines(&mut self, to_screen: &RectTransform, painter: &eframe::egui::Painter) {
        let mut shapes = Vec::new();
        for line in self.lines.iter() {
            let points: Vec<Pos2> = line.iter().map(|p| to_screen * *p).collect();
            shapes.push(egui::Shape::line(points, self.stroke));
        }
        painter.extend(shapes);
    }

    fn add_line(&mut self, line: &Vec<emath::Pos2>) {
        self.lines.push(line.clone());
    }

    fn clear(&mut self) {
        self.lines.clear();
    }
}
