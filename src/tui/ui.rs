use crate::{domain::Task, tui::user_input};
use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::tui::user_input::{
    Direction::{Left, Right},
    InputMode, UserInput,
};

pub struct Ui {
    pub input_field: UserInput,

    // will be displayed in the UI when error occurs
    pub error_str: Option<String>,
}

pub enum Action {
    None,
    Submit,
    Exit,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            input_field: UserInput::new(),
            error_str: None,
        }
    }

    pub fn process_key_events(self: &mut Self) -> Action {
        if let Some(key) = event::read().unwrap().as_key_press_event() {
            match self.input_field.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        self.input_field.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Action::Exit;
                    }
                    _ => {}
                },
                InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        return Action::Submit;
                    }
                    KeyCode::Char(to_insert) => {
                        if self.error_str.is_none() {
                            self.input_field.enter_char(to_insert);
                        }
                    }
                    KeyCode::Backspace => self.input_field.delete_char(),
                    KeyCode::Left => self.input_field.move_cursor(Left),
                    KeyCode::Right => self.input_field.move_cursor(Right),
                    KeyCode::Esc => {
                        if self.error_str.is_some() {
                            self.error_str = None;
                        } else {
                            self.input_field.input_mode = InputMode::Normal;
                        }
                    }
                    _ => {}
                },
                InputMode::Editing => {}
            }
        }
        Action::None
    }

    pub fn render(
        self: &mut Self,
        frame: &mut Frame,
        tasks: &Vec<Task>,
        active_task: &Option<Task>,
    ) {
        let layout = Layout::vertical([
            Constraint::Min(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ]);
        let [task_list_area, active_task_area, input_area] = frame.area().layout(&layout);

        let items: Vec<ListItem> = tasks
            .iter()
            .map(|task| ListItem::from(format!("[ ] {}", task.title)))
            .collect();

        let list = List::new(items).block(Block::bordered());

        frame.render_widget(list, task_list_area);

        let active_task = match &active_task {
            Some(task) => {
                format!("Actvie task: {:?}", task.title)
            }
            None => "Acrive task: nothing".to_string(),
        };

        let active_item = Paragraph::new(active_task).block(Block::bordered());

        frame.render_widget(active_item, active_task_area);

        self.render_input(frame, input_area);

        if let Some(error_str) = &self.error_str {
            self.render_popup(frame, error_str.as_str());
        }
    }

    fn render_input(self: &Self, frame: &mut Frame, input_area: ratatui::layout::Rect) {
        let input = Paragraph::new(self.input_field.input.as_str())
            .style(match self.input_field.input_mode {
                user_input::InputMode::Normal => ratatui::style::Style::default(),
                user_input::InputMode::Editing => {
                    ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)
                }
            })
            .block(Block::bordered());

        frame.render_widget(input, input_area);

        match self.input_field.input_mode {
            user_input::InputMode::Normal => {}

            #[expect(clippy::cast_possible_truncation)]
            user_input::InputMode::Editing => {
                frame.set_cursor_position(ratatui::layout::Position::new(
                    input_area.x + self.input_field.character_index as u16 + 1,
                    input_area.y + 1,
                ))
            }
        }
    }

    // https://users.rust-lang.org/t/ratatui-how-to-make-a-popup-that-has-width-proportional-to-window-and-height-to-fit-content/137320/4
    fn render_popup(self: &Self, frame: &mut Frame, error: &str) {
        const POPUP_WIDTH_PERCENTAGE: u16 = 50;

        let terminal_area = frame.area();

        let popup_width = terminal_area.width * POPUP_WIDTH_PERCENTAGE / 100;
        let inner_width = popup_width.saturating_sub(2); // Account for borders

        let text_lines = error.len() as f32 / (inner_width as f32 * 0.8); // Rough estimation
        let estimated_height = text_lines.ceil() as u16 + 3; // Add borders
        let popup_height = estimated_height.min(terminal_area.height.saturating_sub(4)); // Limit to terminal height

        let vertical_layout = Layout::vertical([
            Constraint::Percentage((100 - popup_height * 100 / terminal_area.height) / 2),
            Constraint::Length(popup_height),
            Constraint::Percentage((100 - popup_height * 100 / terminal_area.height) / 2),
        ])
        .flex(ratatui::layout::Flex::Legacy)
        .split(terminal_area);

        let popup_area = Layout::horizontal([
            Constraint::Percentage((100 - POPUP_WIDTH_PERCENTAGE) / 2),
            Constraint::Percentage(POPUP_WIDTH_PERCENTAGE),
            Constraint::Percentage((100 - POPUP_WIDTH_PERCENTAGE) / 2),
        ])
        .flex(ratatui::layout::Flex::Legacy)
        .split(vertical_layout[1])[1];

        let paragraph = Paragraph::new(Text::from(error.to_owned() + "\nPress ESC to close."))
            .block(
                Block::bordered()
                    .title(Line::from("Error occured!").centered())
                    .borders(Borders::ALL)
                    .border_style(Style::new().fg(Color::LightRed))
                    .style(Style::new().bg(Color::Black).fg(Color::White)),
            )
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        // Render the popup on top
        frame.render_widget(paragraph, popup_area);
    }
}
