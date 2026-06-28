#![allow(unused)]

mod content;
mod icon;
mod style;

struct App {
    tab: Tab,
    split_state: iced_resizable_split::State,

    tasks: content::Tasks,
    stats: content::Stats,
    config: content::History,

    core: application::Core,
}

#[derive(Clone)]
enum AppMessage {
    TabChanged(Tab),
    AsideResized(iced_resizable_split::State),

    TasksMessage(content::TasksMessage),
    StatsMessage(content::StatsMessage),
    HistoryMessage(content::HistoryMessage),
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum Tab {
    #[default]
    Tasks,
    Stats,
    History,
}

trait Display {
    type Message;

    fn update(&mut self, message: Self::Message, core: &mut application::Core);

    fn view_center(&self, core: &application::Core) -> iced::Element<'_, Self::Message>;

    fn view_aside(&self, core: &application::Core) -> iced::Element<'_, Self::Message>;

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::none()
    }
}

impl Tab {
    const fn name(self) -> &'static str {
        match self {
            Self::Tasks => "TASKS",
            Self::Stats => "STATS",
            Self::History => "HISTORY",
        }
    }

    fn view(self) -> iced::Element<'static, AppMessage> {
        let button = |tab: Self| {
            let text = iced::widget::text(tab.name())
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .center();
            iced::widget::button(text)
                .on_press(AppMessage::TabChanged(tab))
                .width(iced::Length::Fill)
                .padding(3)
                .style(style::tab_button_style(
                    style::Variant::Default,
                    tab == self,
                ))
        };

        iced::widget::row![
            button(Self::Tasks),
            button(Self::Stats),
            button(Self::History),
        ]
        .width(iced::Length::Fill)
        .height(style::TOP_BAR_HEIGHT)
        .into()
    }
}

impl App {
    const fn change_tab(&mut self, tab: Tab) {
        self.tab = tab;
    }

    fn get_current_tab(&self) -> (iced::Element<'_, AppMessage>, iced::Element<'_, AppMessage>) {
        match self.tab {
            Tab::Tasks => (
                self.tasks
                    .view_center(&self.core)
                    .map(AppMessage::TasksMessage),
                self.tasks
                    .view_aside(&self.core)
                    .map(AppMessage::TasksMessage),
            ),
            Tab::Stats => (
                self.stats
                    .view_center(&self.core)
                    .map(AppMessage::StatsMessage),
                self.stats
                    .view_aside(&self.core)
                    .map(AppMessage::StatsMessage),
            ),
            Tab::History => (
                self.config
                    .view_center(&self.core)
                    .map(AppMessage::HistoryMessage),
                self.config
                    .view_aside(&self.core)
                    .map(AppMessage::HistoryMessage),
            ),
        }
    }
}

impl App {
    fn new() -> Self {
        Self {
            tab: Tab::default(),
            split_state: iced_resizable_split::State::new(0.3, 0.1, 0.8),

            tasks: content::Tasks::new(),
            stats: content::Stats::new(),
            config: content::History::new(),

            core: application::Core::new().expect("Failed to start App"),
        }
    }

    fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::TabChanged(tab) => self.change_tab(tab),
            AppMessage::AsideResized(new_state) => self.split_state.update(new_state),

            AppMessage::TasksMessage(message) => self.tasks.update(message, &mut self.core),
            AppMessage::StatsMessage(message) => self.stats.update(message, &mut self.core),
            AppMessage::HistoryMessage(message) => self.config.update(message, &mut self.core),
        }
    }

    fn view(&self) -> iced::Element<'_, AppMessage> {
        let top_bar = Tab::view(self.tab);

        let split_state = self.split_state;
        let (main, aside) = self.get_current_tab();

        let main = iced::widget::container(main)
            .height(iced::Length::Fill)
            .width(iced::Length::Fill)
            .style(style::default_panel);

        let horizontal_border =
            iced::widget::rule::horizontal(style::BORDER_WIDTH).style(style::border);
        let aside =
            iced::widget::column![top_bar, horizontal_border, iced::widget::container(aside)]
                .width(iced::Length::Fill)
                .height(iced::Length::Fill);
        let aside = iced::widget::container(aside)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .style(style::accent_panel);

        iced_resizable_split::split_vertical(aside, main, split_state, AppMessage::AsideResized)
            .style(style::split_border)
            .into()
    }

    fn theme(_self: &Self) -> iced::Theme {
        iced::Theme::KanagawaLotus
        // iced::Theme::custom(
        //     String::from("Midnight"),
        //     iced::theme::Palette {
        //         background: iced::Color::BLACK,
        //         text: iced::Color::WHITE,
        //         primary: iced::Color::from_rgb8(198, 167, 97),
        //
        //         success: iced::Color::from_rgb(0.0, 1.0, 0.0),
        //         warning: iced::Color::from_rgb(1.0, 0.8, 0.0),
        //         danger: iced::Color::from_rgb(1.0, 0.0, 0.0),
        //     },
        // )
    }

    fn subscription(&self) -> iced::Subscription<AppMessage> {
        match self.tab {
            Tab::Tasks => self.tasks.subscription().map(AppMessage::TasksMessage),
            Tab::Stats => self.stats.subscription().map(AppMessage::StatsMessage),
            Tab::History => self.config.subscription().map(AppMessage::HistoryMessage),
        }
    }
}

fn main() -> iced::Result {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    iced::application(App::new, App::update, App::view)
        .title("Counter")
        .theme(App::theme)
        .font(include_bytes!(
            "../../assets/fonts/IosevkaCode-SemiBold.ttf"
        ))
        .default_font(iced::Font::with_name("Iosevka Code"))
        .subscription(App::subscription)
        .run()
}
