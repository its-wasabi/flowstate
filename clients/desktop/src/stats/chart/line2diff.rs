// TODO: Make that automatically align vertically from most max and most min y points so it
// vertically always covers entire plot

pub struct ChartLine2diff {
    id_source: &'static str,
    data_a: Vec<application::analytics::Point>,
    data_b: Vec<application::analytics::Point>,
    x: application::analytics::MinMax<f64>,
    y: application::analytics::MinMax<f64>,
}

impl ChartLine2diff {
    pub fn new(
        id_source: &'static str,
        data_a: &[application::analytics::Point],
        data_b: &[application::analytics::Point],
    ) -> Self {
        let mut x_min = f64::INFINITY;
        let mut x_max = f64::NEG_INFINITY;
        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;

        for &point in data_a.iter().chain(data_b.iter()) {
            let (x, y) = (point.x(), point.y());
            x_min = x_min.min(x);
            x_max = x_max.max(x);
            y_min = y_min.min(y);
            y_max = y_max.max(y);
        }

        Self {
            id_source,
            data_a: data_a.to_vec(),
            data_b: data_b.to_vec(),
            x: application::analytics::MinMax {
                min: x_min,
                max: x_max,
            },
            y: application::analytics::MinMax {
                min: y_min,
                max: y_max,
            },
        }
    }

    fn plot_config(&self) -> egui_plot::Plot<'_> {
        egui_plot::Plot::new(self.id_source)
            .allow_scroll(egui::Vec2b::new(true, false))
            .allow_drag(egui::Vec2b::new(true, false))
            .allow_boxed_zoom(false)
            .allow_zoom(egui::Vec2b::new(true, false))
            .allow_axis_zoom_drag(egui::Vec2b::new(false, false))
            .show_crosshair(false)
            .show_background(false)
            .show_grid(true)
            .grid_color(super::COLOR_GRID)
            .grid_fade(0.4)
            .default_y_bounds(0.0, self.y.max)
            .default_x_bounds(0.0, self.x.max)
            .clamp_grid(true)
            .show_axes(egui::Vec2b::new(true, true))
            .show_x(true)
            .show_y(true)
            .include_x(self.x.min)
            .include_x(self.x.max)
            .x_grid_spacer(Self::x_grid_spacer)
    }

    fn plot_points_from_points(
        points: &[application::analytics::Point],
    ) -> egui_plot::PlotPoints<'_> {
        if points.is_empty() {
            return egui_plot::PlotPoints::default();
        }

        let arr_slice: &[[f64; 2]] =
            unsafe { std::slice::from_raw_parts(points.as_ptr() as *const [f64; 2], points.len()) };

        let vec: Vec<[f64; 2]> = arr_slice.to_vec();

        egui_plot::PlotPoints::from(vec)
    }

    pub fn show_plot(&self, ui: &mut egui::Ui) {
        if self.data_a.is_empty() || self.data_b.is_empty() {
            ui.centered_and_justified(|ui| ui.label("NO DATA"));
            return;
        }

        self.plot_config().show(ui, |plot_ui| {
            Self::plot_horizontal_bound(plot_ui, &self.x);

            // let b = plot_ui.plot_bounds();
            // let x_min = b.min()[0];
            // let x_max = b.max()[0];
            // let x = application::analytics::MinMax {
            //     min: x_min,
            //     max: x_max,
            // };
            //
            // let view_a = super::visible_slice(&self.data_a, x);
            // let view_b = super::visible_slice(&self.data_b, x);
            //
            // let view_smooth_a = super::smooth(view_a);
            // let view_smooth_b = super::smooth(view_b);

            plot_ui.line(
                egui_plot::Line::new("A", Self::plot_points_from_points(&self.data_a))
                    .color(super::COLOR_TODO)
                    .width(super::LINE_WIDTH),
            );

            plot_ui.line(
                egui_plot::Line::new("B", Self::plot_points_from_points(&self.data_b))
                    .color(super::COLOR_DONE)
                    .width(super::LINE_WIDTH),
            );

            //
            // let Some(pointer) = plot_ui.pointer_coordinate() else {
            //     return;
            // };
            //
            // let x = pointer.x;
            //
            // let i = (pointer.x - self.x_min).round() as isize;
            // let i = i.clamp(0, self.data_a.len() as isize - 1) as usize;
            // let raw_y_a = self.data_a[i][1];
            // let raw_y_b = self.data_b[i][1];
            //
            // let (Some(smooth_y_a), Some(smooth_y_b)) = (
            //     super::sample(&view_smooth_a, pointer.x),
            //     super::sample(&view_smooth_b, pointer.x),
            // ) else {
            //     return;
            // };
            //
            // let diff = raw_y_a - raw_y_b;
            //
            // let y_mid = (smooth_y_a + smooth_y_b) * 0.5;
            // let y_label_a = smooth_y_a + super::LABEL_DIST;
            // let y_label_b = smooth_y_b - super::LABEL_DIST;
            //
            // let show_diff = diff.abs() >= 0.2;
            // let bounds = plot_ui.plot_bounds();
            // let mut gaps = vec![
            //     (smooth_y_a - super::CROSS_PAD, y_label_a + super::LABEL_PAD), // A: dot + label
            //     (y_label_b - super::LABEL_PAD, smooth_y_b + super::CROSS_PAD), // B: label + dot
            // ];
            // if show_diff {
            //     gaps.push((y_mid - super::LABEL_PAD, y_mid + super::LABEL_PAD));
            // }
            // super::segmented_vline(
            //     plot_ui,
            //     x,
            //     bounds.min()[1],
            //     bounds.max()[1],
            //     &mut gaps,
            //     super::VLINE_COLOR,
            // );
            //
            // plot_ui.text(egui_plot::Text::new(
            //     "label_a",
            //     egui_plot::PlotPoint::new(x, y_label_a),
            //     egui::RichText::new(format!("{raw_y_a:.0}"))
            //         .size(super::LABEL_SIZE)
            //         .color(super::COLOR_TODO)
            //         .strong(),
            // ));
            // plot_ui.text(egui_plot::Text::new(
            //     "label_b",
            //     egui_plot::PlotPoint::new(x, y_label_b),
            //     egui::RichText::new(format!("{raw_y_b:.0}"))
            //         .size(super::LABEL_SIZE)
            //         .color(super::COLOR_DONE)
            //         .strong(),
            // ));
            //
            // if show_diff {
            //     plot_ui.text(egui_plot::Text::new(
            //         "ld",
            //         egui_plot::PlotPoint::new(x, y_mid),
            //         egui::RichText::new(format!("{diff:.0}"))
            //             .size(super::LABEL_SIZE)
            //             .color(egui::Color32::WHITE),
            //     ));
            // }
            //
            // plot_ui.points(
            //     egui_plot::Points::new("point_a", vec![[x, smooth_y_a]])
            //         .radius(super::DOT_RADIUS)
            //         .color(super::COLOR_TODO),
            // );
            // plot_ui.points(
            //     egui_plot::Points::new("point_b", vec![[x, smooth_y_b]])
            //         .radius(super::DOT_RADIUS)
            //         .color(super::COLOR_DONE),
            // );
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

    fn plot_horizontal_bound(
        plot_ui: &mut egui_plot::PlotUi,
        x: &application::analytics::MinMax<f64>,
    ) {
        let bounds = plot_ui.plot_bounds();
        let view_x_range = bounds.max()[0] - bounds.min()[0];

        if view_x_range > x.max - x.min {
            plot_ui.set_plot_bounds(egui_plot::PlotBounds::from_min_max(
                [x.min, bounds.min()[1]],
                [x.max, bounds.max()[1]],
            ));
        } else {
            let new_min = bounds.min()[0].clamp(x.min, x.max - view_x_range);
            if (new_min - bounds.min()[0]).abs() > f64::EPSILON {
                plot_ui.set_plot_bounds(egui_plot::PlotBounds::from_min_max(
                    [new_min, bounds.min()[1]],
                    [new_min + view_x_range, bounds.max()[1]],
                ));
            }
        }
    }
}
