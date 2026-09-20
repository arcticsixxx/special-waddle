use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    widgets::{Block, List, ListItem, Paragraph},
};

use crate::tui::{
    app::TuiApp,
    user_input::{
        Direction::{Left, Right},
        InputMode, UserInput,
    },
};
use crate::{storage::app_repository::AppRepository, tui::user_input};

pub struct Ui {
    pub input_field: UserInput,
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
                    KeyCode::Char(to_insert) => self.input_field.enter_char(to_insert),
                    KeyCode::Backspace => self.input_field.delete_char(),
                    KeyCode::Left => self.input_field.move_cursor(Left),
                    KeyCode::Right => self.input_field.move_cursor(Right),
                    KeyCode::Esc => self.input_field.input_mode = InputMode::Normal,
                    _ => {}
                },
                InputMode::Editing => {}
            }
        }
        Action::None
    }

    pub fn render<R: AppRepository>(self: &Self, frame: &mut Frame, app: &TuiApp<R>) {
        let layout = Layout::vertical([
            Constraint::Min(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ]);
        let [task_list_area, active_task_area, input_area] = frame.area().layout(&layout);

        let items: Vec<ListItem> = app
            .tasks
            .iter()
            .map(|task| ListItem::from(format!("[ ] {}", task.title)))
            .collect();

        let list = List::new(items).block(Block::bordered());

        frame.render_widget(list, task_list_area);

        let active_task = match &app.active_task {
            Some(task) => {
                format!("Actvie task: {:?}", task.title)
            }
            None => "Acrive task: nothing".to_string(),
        };

        let active_item = Paragraph::new(active_task).block(Block::bordered());

        frame.render_widget(active_item, active_task_area);

        self.render_input(frame, input_area);
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

    pub fn show_error(self: &Self, error: &str) {}
}
