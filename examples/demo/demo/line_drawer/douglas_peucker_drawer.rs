use eframe::egui;
use egui::{Color32, Stroke};
use emath::{Pos2, RectTransform};
use line_simplification::{douglas_peucker::douglas_peucker, types::Point};

use super::{draw_threshold_setting, LineDrawer, THRESHOLD_DEFAULT};

const LINE_COLOR: Color32 = Color32::RED;
const LINE_WIDTH: f32 = 1.0;

pub struct DouglasPeuckerDrawer {
    stroke: Stroke,
    threshold: f32,
    lines: Vec<Vec<Point>>,
}

impl Default for DouglasPeuckerDrawer {
    fn default() -> Self {
        Self {
            stroke: Stroke::new(LINE_WIDTH, LINE_COLOR),
            threshold: THRESHOLD_DEFAULT,
            lines: vec![],
        }
    }
}

impl LineDrawer for DouglasPeuckerDrawer {
    fn draw_settings(&mut self, ui: &mut eframe::egui::Ui) {
        ui.group(|ui| {
            ui.heading("Douglas Peucker Line");
            egui::stroke_ui(ui, &mut self.stroke, "Stroke");
            draw_threshold_setting(ui, &mut self.threshold, "Threshold");
        });
    }

    fn draw_lines(&mut self, to_screen: &RectTransform, painter: &eframe::egui::Painter) {
        let mut shapes = Vec::new();
        for line in self.lines.iter() {
            let line = douglas_peucker(&line, self.threshold);
            let points: Vec<Pos2> = line.iter().map(|p| to_screen * p.into()).collect();
            shapes.push(egui::Shape::line(points, self.stroke));
        }
        painter.extend(shapes);
    }

    fn add_line(&mut self, line: &Vec<emath::Pos2>) {
        let line: Vec<Point> = line.iter().map(|p| p.into()).collect();
        self.lines.push(line);
    }

    fn clear(&mut self) {
        self.lines.clear();
    }
}
