pub mod line2diff;

const COLOR_GRID: egui::Color32 = egui::Color32::from_gray(60);

const COLOR_TODO: egui::Color32 = egui::Color32::from_rgb(0xFF, 0xB0, 0x20);
const COLOR_DONE: egui::Color32 = egui::Color32::from_rgb(0x00, 0xFF, 0x9C);

const LINE_WIDTH: f32 = 3.4;

fn visible_slice(data: &[(f64, f64)], x: application::analytics::MinMax<f64>) -> &[(f64, f64)] {
    let start = data.partition_point(|p| p.0 < x.min);
    let end = data.partition_point(|p| p.0 <= x.max);

    let start = start.saturating_sub(2);
    let end = (end + 2).min(data.len());

    &data[start..end]
}
