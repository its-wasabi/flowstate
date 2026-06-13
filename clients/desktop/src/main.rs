#![allow(unused)]

mod content;
mod style;
mod widgets;

struct App {
    tab: Tab,
    split_ratio: f32,

    tasks: content::Tasks,
    stats: content::Stats,
    config: content::History,
}

#[derive(Clone)]
enum AppMessage {
    TabChanged(Tab),
    AsideResized(f32),

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

    fn update(&mut self, message: Self::Message);

    fn view_center(&self) -> iced::Element<'_, Self::Message>;

    fn view_aside(&self) -> iced::Element<'_, Self::Message>;

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
                .style(style::tab_button_style(tab == self))
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
                self.tasks.view_center().map(AppMessage::TasksMessage),
                self.tasks.view_aside().map(AppMessage::TasksMessage),
            ),
            Tab::Stats => (
                self.stats.view_center().map(AppMessage::StatsMessage),
                self.stats.view_aside().map(AppMessage::StatsMessage),
            ),
            Tab::History => (
                self.config.view_center().map(AppMessage::HistoryMessage),
                self.config.view_aside().map(AppMessage::HistoryMessage),
            ),
        }
    }
}

impl App {
    fn new() -> Self {
        Self {
            tab: Tab::default(),
            split_ratio: 0.25,

            tasks: content::Tasks::new(),
            stats: content::Stats::new(),
            config: content::History::new(),
        }
    }

    fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::TabChanged(tab) => self.change_tab(tab),
            AppMessage::AsideResized(new_ratio) => self.split_ratio = new_ratio,

            AppMessage::TasksMessage(message) => self.tasks.update(message),
            AppMessage::StatsMessage(message) => self.stats.update(message),
            AppMessage::HistoryMessage(message) => self.config.update(message),
        }
    }

    fn view(&self) -> iced::Element<'_, AppMessage> {
        let top_bar = Tab::view(self.tab);

        let (main, aside) = self.get_current_tab();

        let main = iced::widget::container(main)
            .height(iced::Length::Fill)
            .width(iced::Length::Fill)
            .style(style::default_panel);

        let horizontal_border = widgets::border_horizontal(1);
        let aside =
            iced::widget::column![top_bar, horizontal_border, iced::widget::container(aside)]
                .width(iced::Length::Fill)
                .height(iced::Length::Fill);
        let aside = iced::widget::container(aside)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .style(style::accent_panel);

        widgets::SplitVertical::new(
            aside,
            main,
            self.split_ratio,
            0.05,
            0.10,
            AppMessage::AsideResized,
        )
        .into()
    }

    fn theme(_self: &Self) -> iced::Theme {
        iced::Theme::custom(
            String::from("Midnight"),
            iced::theme::Palette {
                background: iced::Color::BLACK,
                text: iced::Color::WHITE,
                primary: iced::Color::from_rgb(0.8, 0.8, 0.8),

                success: iced::Color::from_rgb(0.0, 1.0, 0.0),
                warning: iced::Color::from_rgb(1.0, 0.8, 0.0),
                danger: iced::Color::from_rgb(1.0, 0.0, 0.0),
            },
        )
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
