pub struct Chart {
    data: ChartKind,
    minmax: MinMax,
}

enum ChartKind {
    Task(application::analytics::TasksChart),
    Work(application::analytics::WorkChart),
}

#[derive(Default)]
struct MinMax {
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
}

#[derive(Debug, Clone)]
pub enum ChartMessage {
    Move,
    Zoom,
}

impl Chart {
    pub(super) fn new_work() -> Self {
        Self {
            data: ChartKind::Work(application::analytics::WorkChart::new()),
            minmax: MinMax::default(),
        }
    }

    pub(super) fn new_task() -> Self {
        Self {
            data: ChartKind::Task(application::analytics::TasksChart::new()),
            minmax: MinMax::default(),
        }
    }

    pub(super) fn sync(&mut self, tree: &application::tree::Tree) {
        match &mut self.data {
            ChartKind::Task(chart) => chart.sync(tree),
            ChartKind::Work(chart) => chart.sync(tree),
        }
    }

    pub(super) fn update(&mut self, message: ChartMessage) {
        match message {
            ChartMessage::Move => todo!("IMPL (move)"),
            ChartMessage::Zoom => todo!("IMPL (zoom)"),
        }
    }

    pub(super) fn view<'a>(&self) -> iced::Element<'a, ChartMessage> {
        iced::widget::responsive(|size| iced::widget::text!("{size:?}").into()).into()
    }
}
