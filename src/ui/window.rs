use crate::browser::{Tab, WebViewManager};
use crate::utils::BrowserConfig;
use iced::{
    widget::{button, container, text, text_input, Column, Row},
    {Application, Background, Color, Command, Element, Length, Theme}, {Font, Settings},
};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub struct BagelApp {
    pub webview_manager: Arc<Mutex<WebViewManager>>,
    pub tabs: Vec<Tab>,
    pub active_tab_index: usize,
    pub address_bar_value: String,
    pub is_loading: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    NewTab,
    CloseTab(usize),
    SwitchTab(usize),
    AddressBarChanged(String),
    NavigateTo(String),
    GoBack,
    GoForward,
    Reload,
    AddBookmark,
    ShowMenu,
}

impl Application for BagelApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = (BrowserConfig,);

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        let config = flags.0;
        let webview_manager = Arc::new(Mutex::new(WebViewManager::new(config)));

        // start with a fresh tab
        // later on we can configure with onboarding whether
        // to show a home page or not or empty tab
        let initial_tab = Tab {
            id: Uuid::new_v4(),
            title: "New Tab".to_string(),
            url: "bagel://home".to_string(),
            favicon: None,
            is_loading: false,
            can_go_back: false,
            can_go_forward: false,
            created_at: chrono::Utc::now(),
            last_accessed: chrono::Utc::now(),
            is_pinned: false,
            is_muted: false,
        };

        (
            BagelApp {
                webview_manager,
                tabs: vec![initial_tab],
                active_tab_index: 0,
                address_bar_value: String::new(),
                is_loading: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Bagel Browser".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NewTab => {
                let new_tab = Tab {
                    id: Uuid::new_v4(),
                    title: "New Tab".to_string(),
                    url: "bagel://home".to_string(), //possible home screen to show the goods
                    favicon: None,
                    is_loading: false,
                    can_go_back: false,
                    can_go_forward: false,
                    created_at: chrono::Utc::now(),
                    last_accessed: chrono::Utc::now(),
                    is_pinned: false,
                    is_muted: false,
                };

                self.tabs.push(new_tab);
                self.active_tab_index = self.tabs.len() - 1;
                self.address_bar_value.clear();
            }
            Message::CloseTab(index) => {
                if self.tabs.len() > 1 && index < self.tabs.len() {
                    self.tabs.remove(index);
                    if self.active_tab_index >= index && self.active_tab_index > 0 {
                        self.active_tab_index -= 1;
                    }
                    if self.active_tab_index >= self.tabs.len() {
                        self.active_tab_index = self.tabs.len() - 1;
                    }
                }
            }
            Message::SwitchTab(index) => {
                if index < self.tabs.len() {
                    self.active_tab_index = index;
                    self.address_bar_value = self.tabs[index].url.clone();
                }
            }
            Message::AddressBarChanged(value) => {
                self.address_bar_value = value;
            }
            Message::NavigateTo(url) => {
                if self.active_tab_index < self.tabs.len() {
                    self.tabs[self.active_tab_index].url = url.clone();
                    self.tabs[self.active_tab_index].title = "Loading...".to_string();
                    self.tabs[self.active_tab_index].is_loading = true;

                    // Simulate navigation completion
                    self.tabs[self.active_tab_index].is_loading = false;
                    self.tabs[self.active_tab_index].title = self.extract_title_from_url(&url);
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let toolbar = self.create_toolbar();
        let content = self.create_content();

        Column::new()
            .push(toolbar)
            .push(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

impl BagelApp {
    fn create_toolbar(&self) -> Element<Message> {
        // Tab bar
        let mut tab_row = Row::new().spacing(4);

        for (index, tab) in self.tabs.iter().enumerate() {
            let is_active = index == self.active_tab_index;
            let tab_title = if tab.title.len() > 12 {
                format!("{}...", &tab.title[..12])
            } else {
                tab.title.clone()
            };

            let tab_content = Row::new()
                .push(
                    text(&tab_title)
                        .size(12)
                        .font(Font::with_name("Ubuntu"))
                        .style(if is_active {
                            Color::from_rgb(0.95, 0.95, 0.95)
                        } else {
                            Color::from_rgb(0.7, 0.7, 0.7)
                        }),
                )
                .push(
                    button("×")
                        .on_press(Message::CloseTab(index))
                        .padding([2, 4])
                        .style(iced::theme::Button::Text),
                )
                .spacing(6)
                .align_items(iced::Alignment::Center);

            let tab_button = button(tab_content)
                .on_press(Message::SwitchTab(index))
                .padding([6, 16])
                .style(if is_active {
                    iced::theme::Button::Primary
                } else {
                    iced::theme::Button::Secondary
                });

            tab_row = tab_row.push(tab_button);
        }

        let new_tab_btn = button(text("+").font(Font::with_name("Ubuntu")).size(14))
            .on_press(Message::NewTab)
            .padding([6, 12])
            .style(iced::theme::Button::Secondary);

        tab_row = tab_row.push(new_tab_btn);

        // Navigation buttons
        let nav_buttons = Row::new()
            .push(
                button(text("‹").font(Font::with_name("Ubuntu")).size(14))
                    .on_press(Message::GoBack)
                    .padding([6, 10])
                    .style(iced::theme::Button::Secondary),
            )
            .push(
                button(text("›").font(Font::with_name("Ubuntu")).size(14))
                    .on_press(Message::GoForward)
                    .padding([6, 10])
                    .style(iced::theme::Button::Secondary),
            )
            .push(
                button(text("↻").font(Font::with_name("Ubuntu")).size(14)) // ubuntu doesnt like this symbol
                    .on_press(Message::Reload)
                    .padding([6, 10])
                    .style(iced::theme::Button::Secondary),
            )
            .spacing(4);

        // Address bar
        let address_bar = text_input("Enter URL or search...", &self.address_bar_value)
            .on_input(Message::AddressBarChanged)
            .on_submit(Message::NavigateTo(self.address_bar_value.clone()))
            .padding(8)
            .font(Font::with_name("Ubuntu"))
            .size(14)
            .width(Length::Fill);

        // Toolbar actions
        let actions = Row::new()
            .push(
                button(text("☆").font(Font::with_name("Ubuntu")).size(14)) // or this one
                    .on_press(Message::AddBookmark)
                    .padding([6, 10])
                    .style(iced::theme::Button::Secondary),
            )
            .push(
                button(text("≡").font(Font::with_name("Ubuntu")).size(14)) // or this one
                    .on_press(Message::ShowMenu)
                    .padding([6, 10])
                    .style(iced::theme::Button::Secondary),
            )
            .spacing(4);

        // put everything in the toolbar
        // see what i was talking about in ./ui/toolbar.rs
        let toolbar = Row::new()
            .push(tab_row)
            .push(nav_buttons)
            .push(address_bar)
            .push(actions)
            .spacing(8)
            .padding(8)
            .width(Length::Fill);

        container(toolbar)
            .width(Length::Fill)
            .style(container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.15, 0.15, 0.18))),
                border: iced::Border {
                    radius: 0.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.25, 0.25, 0.3),
                },
                shadow: iced::Shadow::default(),
                text_color: Some(Color::from_rgb(0.9, 0.9, 0.9)),
            })
            .into()
    }

    fn create_content(&self) -> Element<Message> {
        let current_tab = &self.tabs[self.active_tab_index];

        if current_tab.url == "bagel://home" {
            self.create_home_page()
        } else {
            self.create_web_content(&current_tab.url)
        }
    }

    fn create_home_page(&self) -> Element<Message> {
        let logo = Column::new()
            .push(
                text("🥯 Bagel Browser") // bagel dont render; i sad
                    .size(28)
                    .font(Font::with_name("Ubuntu"))
                    .style(Color::from_rgb(0.9, 0.9, 0.9)),
            )
            .push(
                text("A minimal, performant web browser")
                    .size(14)
                    .font(Font::with_name("Ubuntu"))
                    .style(Color::from_rgb(0.7, 0.7, 0.7)),
            )
            .align_items(iced::Alignment::Center)
            .spacing(8);

        let quick_access = Column::new()
            .push(
                text("Quick Access")
                    .size(16)
                    .font(Font::with_name("Ubuntu"))
                    .style(Color::from_rgb(0.8, 0.8, 0.8)),
            )
            .push(
                text("No bookmarks yet")
                    .size(12)
                    .font(Font::with_name("Ubuntu"))
                    .style(Color::from_rgb(0.6, 0.6, 0.6)),
            )
            .spacing(8);

        let recent_history = Column::new()
            .push(
                text("Recently Visited")
                    .size(16)
                    .font(Font::with_name("Ubuntu"))
                    .style(Color::from_rgb(0.8, 0.8, 0.8)),
            )
            .push(
                text("No history yet")
                    .size(12)
                    .font(Font::with_name("Ubuntu"))
                    .style(Color::from_rgb(0.6, 0.6, 0.6)),
            )
            .spacing(8);

        let content = Column::new()
            .push(logo)
            .push(quick_access)
            .push(recent_history)
            .spacing(32)
            .padding(32)
            .align_items(iced::Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.12, 0.12, 0.15))),
                border: iced::Border {
                    radius: 0.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.25, 0.25, 0.3),
                },
                shadow: iced::Shadow::default(),
                text_color: Some(Color::from_rgb(0.9, 0.9, 0.9)),
            })
            .into()
    }

    fn create_web_content(&self, _url: &str) -> Element<Message> {
        let content = Column::new()
            .push(
                text("Web Content Area")
                    .size(24)
                    .font(Font::with_name("Ubuntu"))
                    .style(Color::from_rgb(0.8, 0.8, 0.8)),
            )
            .push(
                text("This would show the actual web page")
                    .size(14)
                    .font(Font::with_name("Ubuntu"))
                    .style(Color::from_rgb(0.6, 0.6, 0.6)),
            )
            .spacing(16)
            .align_items(iced::Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.13))),
                border: iced::Border {
                    radius: 0.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: iced::Shadow::default(),
                text_color: Some(Color::from_rgb(0.9, 0.9, 0.9)),
            })
            .into()
    }

    fn extract_title_from_url(&self, url: &str) -> String {
        if let Ok(parsed_url) = url::Url::parse(url) {
            parsed_url.host_str().unwrap_or("New Tab").to_string()
        } else {
            "New Tab".to_string()
        }
    }
}

pub fn run_app(config: BrowserConfig) -> iced::Result {
    BagelApp::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(1200.0, 800.0),
            min_size: Some(iced::Size::new(800.0, 600.0)),
            ..Default::default()
        },
        flags: (config,),
        ..Default::default()
    })
}
