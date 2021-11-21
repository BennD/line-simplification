pub mod line_drawer;

use eframe::{
    egui::{self, emath, Color32, Direction, Layout, Pos2, Rect, Response, Sense, Stroke, Ui},
    epi,
};

use emath::Vec2;
use line_drawer::{DouglasPeuckerDrawer, LineDrawer, OriginalDrawer};

pub struct Demo {
    line: Vec<Pos2>,
    drawers: Vec<Box<dyn LineDrawer>>,
    stroke: Stroke,
}

impl Default for Demo {
    fn default() -> Self {
        Self {
            line: vec![],
            drawers: vec![
                Box::new(OriginalDrawer::default()),
                Box::new(DouglasPeuckerDrawer::default()),
            ],
            stroke: Stroke::new(0.2, Color32::WHITE),
        }
    }
}

impl epi::App for Demo {
    fn name(&self) -> &str {
        "Demo"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        // Demo settings
        egui::Window::new("Settings")
            .default_size(Vec2 { x: 1.0, y: 1.0 }) // ensure windows starts the minimally required size
            .resizable(false)
            .show(ctx, |ui| {
                for drawer in self.drawers.iter_mut() {
                    drawer.draw_settings(ui);
                }

                ui.with_layout(
                    Layout::centered_and_justified(Direction::LeftToRight),
                    |ui| {
                        if ui.button("Clear Lines").clicked() {
                            for drawer in self.drawers.iter_mut() {
                                drawer.clear();
                            }
                        }
                    },
                );
            });

        // Line Viewer
        egui::CentralPanel::default().show(ctx, |mut ui| {
            self.draw_lines(&mut ui);
        });

        // Resize the native window to be at least the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }
}

impl Demo {
    fn draw_lines(&mut self, ui: &mut Ui) -> Response {
        let (mut response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        let from_screen = to_screen.inverse();

        // Handle lines being drawn
        if let Some(pointer_pos) = response.interact_pointer_pos() {
            // Cursor draged - line is being drawn
            let canvas_pos = from_screen * pointer_pos;
            if self.line.last() != Some(&canvas_pos) {
                self.line.push(canvas_pos);
                response.mark_changed();
            }

            // Draw line
            if self.line.len() >= 2 {
                let points: Vec<Pos2> = self.line.iter().map(|p| to_screen * *p).collect();
                painter.add(egui::Shape::line(points, self.stroke));
            }
        } else if !self.line.is_empty() {
            // Cursor released - line finished
            if self.line.len() >= 2 {
                for drawer in self.drawers.iter_mut() {
                    drawer.add_line(&self.line);
                }
            }
            self.line.clear();
            response.mark_changed();
        }

        // Draw existing lines
        for drawer in self.drawers.iter_mut() {
            drawer.draw_lines(&to_screen, &painter);
        }

        response
    }
}
