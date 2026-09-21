use crate::{
    domain::Task,
    tui::{popup::Popup, user_input},
};
use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    widgets::{Block, List, ListItem, Paragraph},
};

use crate::tui::user_input::{
    Direction::{Left, Right},
    InputMode, UserInput,
};

pub struct Ui {
    pub input_field: UserInput,

    pub popup: Option<Popup>,
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
            popup: None,
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
                        if self.popup.is_none() {
                            return Action::Submit;
                        }
                    }
                    KeyCode::Char(to_insert) => {
                        if self.popup.is_none() {
                            self.input_field.enter_char(to_insert);
                        }
                    }
                    KeyCode::Backspace => self.input_field.delete_char(),
                    KeyCode::Left => self.input_field.move_cursor(Left),
                    KeyCode::Right => self.input_field.move_cursor(Right),
                    KeyCode::Esc => {
                        if self.popup.is_some() {
                            self.popup = None;
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

        frame.render_widget(&self.input_field, input_area);
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

        frame.render_widget(active_item, active_task_area);

        if let Some(popup) = &self.popup {
            let popup_area = self.centered(frame.area(), 40, 10);
            frame.render_widget(popup, popup_area);
        }
    }

    // ai-slop
    fn centered(&self, area: Rect, width: u16, height: u16) -> Rect {
        let [area] = Layout::horizontal([Constraint::Length(width)])
            .flex(Flex::Center)
            .areas(area);
        let [area] = Layout::vertical([Constraint::Length(height)])
            .flex(Flex::Center)
            .areas(area);
        area
    }
}
