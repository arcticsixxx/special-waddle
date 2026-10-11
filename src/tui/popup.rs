use ratatui::{
    layout::{Alignment, Rect},
    prelude::Buffer,
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::tui::user_input::UserInput;

pub struct Popup {
    message: String,
    kind: PopupKind,
}

enum PopupKind {
    Error,
    Input { task_input: UserInput },
}

impl Popup {
    fn new(message: impl Into<String>, kind: PopupKind) -> Self {
        Self {
            message: message.into(),
            kind,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self::new(message, PopupKind::Error)
    }

    pub fn input(message: impl Into<String>) -> Self {
        Self::new(
            message,
            PopupKind::Input {
                task_input: UserInput::new(),
            },
        )
    }
}

impl Widget for &Popup {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let paragraph = Paragraph::new(Text::from(
            self.message.to_owned() + "\nPress ESC to close.",
        ))
        .block(
            Block::bordered()
                .title(Line::from("Error occured!").centered())
                .borders(Borders::ALL)
                .border_style(Style::new().fg(Color::LightRed))
                .style(Style::new().bg(Color::Black).fg(Color::White)),
        )
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

        paragraph.render(area, buf);
    }
}
