// mod chart;

pub struct Stats {
    split_ratio: iced_resizable_split::State,
    // top_chart: chart::Chart,
    // bottom_chart: chart::Chart,
}

#[derive(Debug, Clone)]
pub enum StatsMessage {
    SplitResized(iced_resizable_split::State),
    // TopChart(chart::ChartMessage),
    // BottomChart(chart::ChartMessage),
}

impl Stats {
    pub fn new() -> Self {
        Self {
            split_ratio: iced_resizable_split::State::new(0.5, 0.1, 0.9),
            // top_chart: chart::Chart::new_task(),
            // bottom_chart: chart::Chart::new_work(),
        }
    }

    // pub fn sync(&mut self, tree: &application::tree::Tree) {
    //     self.top_chart.sync(tree);
    //     self.bottom_chart.sync(tree);
    // }
}

impl crate::Display for Stats {
    type Message = StatsMessage;

    fn update(&mut self, message: Self::Message, _core: &mut application::Core) {
        match message {
            StatsMessage::SplitResized(new_state) => self.split_ratio.update(new_state),
            // StatsMessage::TopChart(message) => self.top_chart.update(message),
            // StatsMessage::BottomChart(message) => self.bottom_chart.update(message),
        }
    }

    fn view_center(&self, _core: &application::Core) -> iced::Element<'_, Self::Message> {
        // iced_resizable_split::split_horizontal(
        //     self.top_chart.view().map(StatsMessage::TopChart),
        //     self.bottom_chart.view().map(StatsMessage::BottomChart),
        //     self.split_ratio,
        //     StatsMessage::SplitResized,
        // )
        // .style(crate::style::split_border)
        // .into()

        iced::widget::text("(TODO)").into()
    }

    fn view_aside(&self, _core: &application::Core) -> iced::Element<'_, Self::Message> {
        iced::widget::text("HELLO ASIDE").into()
    }
}
