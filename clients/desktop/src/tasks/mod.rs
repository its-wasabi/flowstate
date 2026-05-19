pub struct Tasks {
    current_task: automerge::ObjId,
}

impl Tasks {
    pub fn new() -> Self {
        Self {
            current_task: automerge::ROOT,
        }
    }
}

impl super::View for Tasks {
    fn main(&mut self, ui: &mut egui::Ui, core: &mut application::Core) {
        ui.heading(format!(
            "USING FLOWSTATE {}.{}.{} WITH {}.{}.{} UI (FROM MAIN)",
            application::APP_VERSION.0,
            application::APP_VERSION.1,
            application::APP_VERSION.2,
            super::UI_VERSION.0,
            super::UI_VERSION.1,
            super::UI_VERSION.2,
        ));
    }
    fn aside(&mut self, ui: &mut egui::Ui, core: &mut application::Core) {
        ui.label("ASIDE");
    }
}
