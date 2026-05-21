pub mod line2diff;

const COLOR_GRID: egui::Color32 = egui::Color32::from_gray(60);

const COLOR_TODO: egui::Color32 = egui::Color32::from_rgb(0xFF, 0xB0, 0x20);
const COLOR_DONE: egui::Color32 = egui::Color32::from_rgb(0x00, 0xFF, 0x9C);

const LINE_WIDTH: f32 = 3.4;

const VLINE_COLOR: egui::Color32 = egui::Color32::from_rgb(0xff, 0xff, 0xff);
const VLINE_WIDTH: f32 = 1.0;
const VLINE_MIN_SEG: f64 = 0.05; // segments shorter than this are skipped (avoids stray dots)

const CROSS_PAD: f64 = 0.2; // gap between vline end and intersection dot
const LABEL_PAD: f64 = 0.2; // gap between vline end and number label
const LABEL_DIST: f64 = 0.4; // distance from intersection dot to its number label

const LABEL_SIZE: f32 = 16.0;
const DOT_RADIUS: f32 = 5.0;

fn visible_slice<'a>(
    data: &'a [(f64, f64)],
    x: application::analytics::MinMax<f64>,
) -> &'a [(f64, f64)] {
    let start = data.partition_point(|p| p.0 < x.min);
    let end = data.partition_point(|p| p.0 <= x.max);

    let start = start.saturating_sub(2);
    let end = (end + 2).min(data.len());

    &data[start..end]
}

fn sample(curve: &[[f64; 2]], x: f64) -> Option<f64> {
    curve.windows(2).find_map(|w| {
        let (x0, y0) = (w[0][0], w[0][1]);
        let (x1, y1) = (w[1][0], w[1][1]);
        (x >= x0 && x <= x1).then(|| ((x - x0) / (x1 - x0)).mul_add(y1 - y0, y0))
    })
}

// fn sample_raw(data: &[[f64; 2]], x: f64) -> Option<f64> {
//     let i = data.partition_point(|p| p[0] < x);
//
//     if i == 0 || i >= data.len() {
//         return None;
//     }
//
//     let (x0, y0) = data[i - 1];
//     let (x1, y1) = data[i];
//
//     let t = (x - x0) / (x1 - x0);
//     Some(y0 + t * (y1 - y0))
// }

fn segmented_vline(
    plot_ui: &mut egui_plot::PlotUi,
    x: f64,
    y_min: f64,
    y_max: f64,
    gaps: &mut [(f64, f64)],
    color: egui::Color32,
) {
    gaps.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let draw = |plot_ui: &mut egui_plot::PlotUi, tag: &str, y0: f64, y1: f64| {
        if y1 - y0 > VLINE_MIN_SEG {
            plot_ui.line(
                egui_plot::Line::new(tag, egui_plot::PlotPoints::from(vec![[x, y0], [x, y1]]))
                    .color(color)
                    .width(VLINE_WIDTH)
                    .allow_hover(false),
            );
        }
    };

    let mut cursor = y_min;
    for (i, &(gap_lo, gap_hi)) in gaps.iter().enumerate() {
        draw(
            plot_ui,
            &format!("_vl{i}"),
            cursor,
            gap_lo.clamp(y_min, y_max),
        );
        cursor = cursor.max(gap_hi.clamp(y_min, y_max));
    }
    draw(plot_ui, "_vl_end", cursor, y_max);
}
