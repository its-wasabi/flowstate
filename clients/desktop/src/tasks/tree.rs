// TODO: Add icons to tree as customization option

pub struct TreeState;

impl TreeState {
    pub const fn new() -> Self {
        Self
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        tree: &application::tree::Tree,
        current_task: &mut automerge::ObjId,
    ) {
        if let Ok(children) = tree.get_children(&automerge::ObjId::Root) {
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                for (child_id, _) in children {
                    Self::render_node(ui, tree, &child_id, current_task, 0);
                }
            });
        }
    }

    fn render_node(
        ui: &mut egui::Ui,
        tree: &application::tree::Tree,
        id: &automerge::ObjId,
        current_task: &mut automerge::ObjId,
        depth: usize,
    ) {
        let Ok(node_data) = tree.get_node(id) else {
            return;
        };
        let children = tree.get_children(id).unwrap_or_default();
        let has_children = !children.is_empty();
        let is_selected = id == current_task;

        let collapse_id = ui.make_persistent_id(id);
        let mut is_open = ui.data(|d| d.get_temp::<bool>(collapse_id).unwrap_or(false));

        // 1. Allocate absolute edge-to-edge bounding rect
        let row_height = 24.0;
        let row_rect = egui::Rect::from_min_max(
            egui::pos2(ui.max_rect().left(), ui.cursor().top()),
            egui::pos2(ui.max_rect().right(), ui.cursor().top() + row_height),
        );

        let response = ui.allocate_rect(row_rect, egui::Sense::click());
        let mut arrow_center_x = None;

        if ui.is_rect_visible(row_rect) {
            let (bg_color, text_color) = if is_selected {
                (Some(egui::Color32::WHITE), egui::Color32::BLACK)
            } else if response.hovered() {
                (
                    Some(ui.visuals().widgets.hovered.bg_fill),
                    ui.visuals().widgets.hovered.fg_stroke.color,
                )
            } else {
                (None, ui.visuals().widgets.inactive.text_color())
            };

            if let Some(bg) = bg_color {
                ui.painter().rect_filled(row_rect, 0.0, bg);
            }

            let arrow_font_id = egui::FontId::proportional(16.0);
            let text_font_id = egui::FontId::proportional(13.0);

            let arrow_width = row_height;
            let arrow_x = (depth as f32).mul_add(14.0, row_rect.left());
            let arrow_rect = egui::Rect::from_min_max(
                egui::pos2(arrow_x, row_rect.top()),
                egui::pos2(arrow_x + arrow_width, row_rect.bottom()),
            );
            arrow_center_x = Some(arrow_rect.center().x);

            if has_children {
                let symbol = if is_open { "⏷" } else { "⏵" };

                ui.painter().text(
                    arrow_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    symbol,
                    arrow_font_id,
                    text_color,
                );
            }

            // 3. Keep text padding uniform and stable
            // Fix: Declare text_pos FIRST, then paint the text exactly ONCE
            let text_pos = egui::pos2(arrow_rect.right() + 4.0, row_rect.center().y);
            ui.painter().text(
                text_pos,
                egui::Align2::LEFT_CENTER,
                if node_data.name.is_empty() {
                    "[no name]"
                } else {
                    &node_data.name
                },
                text_font_id,
                text_color,
            );

            if response.clicked()
                && let Some(pos) = response.interact_pointer_pos()
            {
                let arrow_hitbox = egui::Rect::from_min_max(
                    row_rect.left_top(),
                    egui::pos2(arrow_rect.right(), row_rect.bottom()),
                );
                if has_children && arrow_hitbox.contains(pos) {
                    is_open = !is_open;
                    ui.data_mut(|d| d.insert_temp(collapse_id, is_open));
                } else {
                    *current_task = id.clone();
                }
            }
        }

        // 4. Render children subtrees
        if has_children && is_open {
            let start_y = row_rect.bottom();

            for (child_id, _) in children {
                Self::render_node(ui, tree, &child_id, current_task, depth + 1);
            }

            let end_y = ui.cursor().top();

            if let Some(line_x) = arrow_center_x {
                let line_start = egui::pos2(line_x, start_y);
                let line_end = egui::pos2(line_x, end_y - (row_height / 2.0));

                let stroke_color = ui.visuals().widgets.noninteractive.bg_stroke.color;
                let bg_painter = ui
                    .painter()
                    .clone()
                    .with_layer_id(egui::LayerId::background());

                bg_painter
                    .line_segment([line_start, line_end], egui::Stroke::new(1.0, stroke_color));
            }
        }
    }
}
