// TODO: Make that automatically align vertically from most max and most min y points

pub struct ChartLine2diff {
    id_source: &'static str,
    // TODO: Store that in Vec<(f64, f64)> rather than Vec<[f64; 2]>
    data_a: Vec<[f64; 2]>,
    data_b: Vec<[f64; 2]>,
    x_min: f64,
    x_max: f64,
    y_max: f64,
}

impl ChartLine2diff {
    pub fn new(id_source: &'static str, data_a: &[[f64; 2]], data_b: &[[f64; 2]]) -> Self {
        let x_min = data_a
            .iter()
            .chain(data_b)
            .map(|point| point[0])
            .fold(f64::INFINITY, f64::min);
        let x_max = data_a
            .iter()
            .chain(data_b)
            .map(|point| point[0])
            .fold(f64::NEG_INFINITY, f64::max);
        let y_max = data_a
            .iter()
            .chain(data_b)
            .map(|point| point[1])
            .fold(f64::NEG_INFINITY, f64::max);

        Self {
            id_source,
            data_a: data_a.to_vec(),
            data_b: data_b.to_vec(),
            x_min,
            x_max,
            y_max,
        }
    }

    pub fn set_data(&mut self, data_a: &[[f64; 2]], data_b: &[[f64; 2]]) {
        let x_min = data_a
            .iter()
            .chain(data_b)
            .map(|point| point[0])
            .fold(f64::INFINITY, f64::min);
        let x_max = data_a
            .iter()
            .chain(data_b)
            .map(|point| point[0])
            .fold(f64::NEG_INFINITY, f64::max);
        let y_max = data_a
            .iter()
            .chain(data_b)
            .map(|point| point[1])
            .fold(f64::NEG_INFINITY, f64::max);

        self.data_a = data_a.to_vec();
        self.data_b = data_b.to_vec();
        self.x_min = x_min;
        self.x_max = x_max;
        self.y_max = y_max;
    }

    fn plot_config(&self) -> egui_plot::Plot<'_> {
        egui_plot::Plot::new(self.id_source)
            .allow_scroll(egui::Vec2b::new(true, false))
            .allow_drag(egui::Vec2b::new(true, false))
            .allow_boxed_zoom(false)
            .allow_zoom(egui::Vec2b::new(true, false))
            .allow_axis_zoom_drag(egui::Vec2b::new(true, false))
            .show_crosshair(false)
            .show_background(false)
            .show_grid(true)
            .grid_color(super::COLOR_GRID)
            .grid_fade(0.4)
            .default_y_bounds(0.0, self.y_max)
            .clamp_grid(true)
            .show_axes(egui::Vec2b::new(true, true))
            .show_x(true)
            .show_y(true)
            .include_x(self.x_min)
            .include_x(self.x_max)
            .x_grid_spacer(Self::x_grid_spacer)
    }

    pub fn show_plot(&self, ui: &mut egui::Ui) {
        if self.data_a.is_empty() || self.data_b.is_empty() {
            ui.centered_and_justified(|ui| ui.label("NO DATA"));
            return;
        }
        self.plot_config().show(ui, |plot_ui| {
            super::plot_horizontal_bound(plot_ui, self.x_min, self.x_max);

            let b = plot_ui.plot_bounds();
            let x_min = b.min()[0];
            let x_max = b.max()[0];
            let view_a = super::visible_slice(&self.data_a, x_min, x_max);
            let view_b = super::visible_slice(&self.data_b, x_min, x_max);
            let view_smooth_a = super::smooth(view_a);
            let view_smooth_b = super::smooth(view_b);

            plot_ui.line(
                egui_plot::Line::new("A", egui_plot::PlotPoints::from(view_smooth_a.clone()))
                    .color(super::COLOR_TODO)
                    .width(super::LINE_WIDTH),
            );
            plot_ui.line(
                egui_plot::Line::new("B", egui_plot::PlotPoints::from(view_smooth_b.clone()))
                    .color(super::COLOR_DONE)
                    .width(super::LINE_WIDTH),
            );

            let Some(pointer) = plot_ui.pointer_coordinate() else {
                return;
            };

            let x = pointer.x;

            let i = (pointer.x - self.x_min).round() as isize;
            let i = i.clamp(0, self.data_a.len() as isize - 1) as usize;
            let raw_y_a = self.data_a[i][1];
            let raw_y_b = self.data_b[i][1];

            let (Some(smooth_y_a), Some(smooth_y_b)) = (
                super::sample(&view_smooth_a, pointer.x),
                super::sample(&view_smooth_b, pointer.x),
            ) else {
                return;
            };

            let diff = raw_y_a - raw_y_b;

            let y_mid = (smooth_y_a + smooth_y_b) * 0.5;
            let y_label_a = smooth_y_a + super::LABEL_DIST;
            let y_label_b = smooth_y_b - super::LABEL_DIST;

            let show_diff = diff.abs() >= 0.2;
            let bounds = plot_ui.plot_bounds();
            let mut gaps = vec![
                (smooth_y_a - super::CROSS_PAD, y_label_a + super::LABEL_PAD), // A: dot + label
                (y_label_b - super::LABEL_PAD, smooth_y_b + super::CROSS_PAD), // B: label + dot
            ];
            if show_diff {
                gaps.push((y_mid - super::LABEL_PAD, y_mid + super::LABEL_PAD));
            }
            super::segmented_vline(
                plot_ui,
                x,
                bounds.min()[1],
                bounds.max()[1],
                &mut gaps,
                super::VLINE_COLOR,
            );

            plot_ui.text(egui_plot::Text::new(
                "label_a",
                egui_plot::PlotPoint::new(x, y_label_a),
                egui::RichText::new(format!("{raw_y_a:.0}"))
                    .size(super::LABEL_SIZE)
                    .color(super::COLOR_TODO)
                    .strong(),
            ));
            plot_ui.text(egui_plot::Text::new(
                "label_b",
                egui_plot::PlotPoint::new(x, y_label_b),
                egui::RichText::new(format!("{raw_y_b:.0}"))
                    .size(super::LABEL_SIZE)
                    .color(super::COLOR_DONE)
                    .strong(),
            ));

            if show_diff {
                plot_ui.text(egui_plot::Text::new(
                    "ld",
                    egui_plot::PlotPoint::new(x, y_mid),
                    egui::RichText::new(format!("{diff:.0}"))
                        .size(super::LABEL_SIZE)
                        .color(egui::Color32::WHITE),
                ));
            }

            plot_ui.points(
                egui_plot::Points::new("point_a", vec![[x, smooth_y_a]])
                    .radius(super::DOT_RADIUS)
                    .color(super::COLOR_TODO),
            );
            plot_ui.points(
                egui_plot::Points::new("point_b", vec![[x, smooth_y_b]])
                    .radius(super::DOT_RADIUS)
                    .color(super::COLOR_DONE),
            );
        });
    }

    fn x_grid_spacer(input: egui_plot::GridInput) -> Vec<egui_plot::GridMark> {
        let (min_x, max_x) = (input.bounds.0, input.bounds.1);
        let start = min_x.floor() as i64;
        let end = max_x.ceil() as i64;
        (start..=end)
            .filter_map(|i| {
                let val = i as f64;
                if val >= min_x && val <= max_x {
                    Some(egui_plot::GridMark {
                        value: val,
                        step_size: 1.0, // Provide a valid f64 value, e.g., 1.0
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}
